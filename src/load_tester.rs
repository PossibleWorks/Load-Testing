use crate::models::{EndpointResult, LoadTestReport, Scenario, ScenarioResult};
use crate::metrics::LatencyMetrics;
use crate::config::LoadTestConfig;
use reqwest::blocking::Client;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use chrono::Local;
use threadpool::ThreadPool;

const MAX_THREADS: usize = 100; // Limit maximum threads to prevent resource exhaustion
const MAX_CONNECTIONS_PER_HOST: usize = 50; // Connection pool limit

pub struct LoadTester {
    config: LoadTestConfig,
    client: Arc<Client>,
    pool: ThreadPool,
}

impl LoadTester {
    pub fn new(config: LoadTestConfig) -> Self {
        // Calculate optimal thread pool size based on scenarios
        let max_concurrency = config.scenarios.iter()
            .map(|s| s.concurrency)
            .max()
            .unwrap_or(1);
        
        let thread_pool_size = (max_concurrency).min(MAX_THREADS);
        
        // Create a client with connection pooling and timeouts
        let client = Client::builder()
            .pool_max_idle_per_host(MAX_CONNECTIONS_PER_HOST)
            .pool_idle_timeout(Some(Duration::from_secs(30)))
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to create HTTP client");
        
        println!("🔧 LoadTester initialized:");
        println!("   Thread pool size: {}", thread_pool_size);
        println!("   Max connections per host: {}", MAX_CONNECTIONS_PER_HOST);
        println!("   Request timeout: 30s");
        
        Self {
            config,
            client: Arc::new(client),
            pool: ThreadPool::new(thread_pool_size),
        }
    }

    pub fn run(&self) -> LoadTestReport {
        let test_start_time = Local::now();
        let overall_start_time = Instant::now();
        let endpoints = self.config.get_endpoints();

        let mut overall_total_requests = 0;
        let mut overall_total_errors = 0;
        let mut overall_latencies = LatencyMetrics::new();
        let mut scenario_results = Vec::new();

        for scenario in &self.config.scenarios {
            println!(
                "\n🚀 Scenario: Concurrency {}, Requests {}",
                scenario.concurrency, scenario.requests
            );

            let scenario_result = self.run_scenario(scenario, &endpoints);
            
            // Add scenario latencies to overall latencies
            for endpoint in &scenario_result.endpoints {
                // We need to reconstruct latencies from the endpoint results
                // This is a simplified approach - in a real implementation, 
                // you might want to store raw latencies differently
                for _ in 0..endpoint.total_requests {
                    overall_latencies.add_latency(endpoint.mean_latency as u64);
                }
            }

            overall_total_requests += scenario_result.total_requests;
            overall_total_errors += scenario_result.failed_requests;
            scenario_results.push(scenario_result);
        }

        let overall_duration = overall_start_time.elapsed();
        let overall_success = overall_total_requests - overall_total_errors;
        let overall_success_rate = if overall_total_requests > 0 {
            (overall_success as f64 / overall_total_requests as f64) * 100.0
        } else {
            0.0
        };

        let overall_rps = if overall_duration.as_secs_f64() > 0.0 {
            overall_total_requests as f64 / overall_duration.as_secs_f64()
        } else {
            0.0
        };

        let overall_mean_latency = overall_latencies.mean();
        let overall_p95_latency = overall_latencies.percentile(95.0);
        let overall_p99_latency = overall_latencies.percentile(99.0);

        self.print_summary(
            overall_total_requests,
            overall_total_errors,
            overall_success_rate,
            overall_rps,
            overall_mean_latency,
            overall_p95_latency,
            overall_duration.as_secs_f64(),
        );

        let test_end_time = Local::now();
        LoadTestReport {
            test_start_time,
            test_end_time,
            total_duration_seconds: overall_duration.as_secs_f64(),
            overall_requests: overall_total_requests,
            overall_errors: overall_total_errors,
            overall_success_rate,
            overall_rps,
            overall_mean_latency,
            overall_p95_latency,
            overall_p99_latency,
            scenarios: scenario_results,
            base_url: self.config.base_url.clone(),
            endpoints_tested: endpoints,
        }
    }

    fn run_scenario(&self, scenario: &Scenario, endpoints: &[String]) -> ScenarioResult {
        let scenario_start_time = Instant::now();
        let scenario_total_requests = Arc::new(Mutex::new(0));
        let scenario_total_errors = Arc::new(Mutex::new(0));
        let scenario_latencies = Arc::new(Mutex::new(LatencyMetrics::new()));
        let mut endpoint_results = Vec::new();

        // Run each endpoint sequentially with shared thread pool to avoid resource exhaustion
        for endpoint in endpoints {
            let scenario_total_requests = Arc::clone(&scenario_total_requests);
            let scenario_total_errors = Arc::clone(&scenario_total_errors);
            let scenario_latencies = Arc::clone(&scenario_latencies);
            let endpoint = endpoint.to_string();
            let url = format!("{}{}", self.config.base_url, endpoint);
            let concurrency = scenario.concurrency;
            let requests = scenario.requests;

            let endpoint_result = self.run_endpoint_test(
                &url,
                &endpoint,
                concurrency,
                requests,
                &scenario_total_requests,
                &scenario_total_errors,
                &scenario_latencies,
            );

            endpoint_results.push(endpoint_result);
        }

        let scenario_duration = scenario_start_time.elapsed();
        let scenario_requests = *scenario_total_requests.lock().unwrap();
        let scenario_errors = *scenario_total_errors.lock().unwrap();
        let scenario_success = scenario_requests - scenario_errors;

        let scenario_success_rate = if scenario_requests > 0 {
            (scenario_success as f64 / scenario_requests as f64) * 100.0
        } else {
            0.0
        };

        let scenario_rps = if scenario_duration.as_secs_f64() > 0.0 {
            scenario_requests as f64 / scenario_duration.as_secs_f64()
        } else {
            0.0
        };

        let scenario_lat = scenario_latencies.lock().unwrap();
        let scenario_mean_latency = scenario_lat.mean();
        let scenario_p95_latency = scenario_lat.percentile(95.0);
        let scenario_p99_latency = scenario_lat.percentile(99.0);

        // Enhanced statistical analysis
        let stats = scenario_lat.statistical_analysis();
        let p95_with_confidence = scenario_lat.percentile_with_confidence(95.0);
        let p99_with_confidence = scenario_lat.percentile_with_confidence(99.0);

        println!("Scenario Total Requests: {}", scenario_requests);
        println!("Scenario Total Errors: {}", scenario_errors);
        println!("Scenario Success Rate: {:.2}%", scenario_success_rate);
        println!("Scenario RPS: {:.2} req/sec", scenario_rps);
        println!("Scenario Mean Latency: {:.2}ms", scenario_mean_latency);
        println!("Scenario P95 Latency: {}ms (α=0.05, Z={:.3}, CI: {:.1}-{:.1}ms)", 
                 scenario_p95_latency, p95_with_confidence.z_score, 
                 p95_with_confidence.confidence_interval_lower, 
                 p95_with_confidence.confidence_interval_upper);
        println!("Scenario P99 Latency: {}ms (α=0.01, Z={:.3}, CI: {:.1}-{:.1}ms)", 
                 scenario_p99_latency, p99_with_confidence.z_score,
                 p99_with_confidence.confidence_interval_lower, 
                 p99_with_confidence.confidence_interval_upper);
        println!("Distribution: {:?} (Skew: {:.3}, Kurtosis: {:.3})", 
                 stats.distribution_type, stats.skewness, stats.kurtosis);
        println!("Scenario Duration: {:.2}s", scenario_duration.as_secs_f64());

        ScenarioResult {
            concurrency: scenario.concurrency,
            total_requests: scenario_requests,
            successful_requests: scenario_success,
            failed_requests: scenario_errors,
            success_rate: scenario_success_rate,
            rps: scenario_rps,
            mean_latency: scenario_mean_latency,
            p95_latency: scenario_p95_latency,
            p99_latency: scenario_p99_latency,
            duration_seconds: scenario_duration.as_secs_f64(),
            endpoints: endpoint_results,
        }
    }

    fn run_endpoint_test(
        &self,
        url: &str,
        endpoint: &str,
        concurrency: usize,
        requests: usize,
        scenario_total_requests: &Arc<Mutex<usize>>,
        scenario_total_errors: &Arc<Mutex<usize>>,
        scenario_latencies: &Arc<Mutex<LatencyMetrics>>,
    ) -> EndpointResult {
        let success_count = Arc::new(Mutex::new(0));
        let fail_count = Arc::new(Mutex::new(0));
        let endpoint_latencies = Arc::new(Mutex::new(LatencyMetrics::new()));
        let status_counts = Arc::new(Mutex::new(HashMap::new()));
        
        // Calculate delay between requests to achieve desired concurrency
        let request_delay = if concurrency > 0 {
            Duration::from_millis((1000 / concurrency.min(100)) as u64) // Limit to 100 RPS max per endpoint
        } else {
            Duration::from_millis(10)
        };
        
        let pending_requests = Arc::new(Mutex::new(0));
        let completed_requests = Arc::new(Mutex::new(0));

        for i in 0..requests {
            // Wait if we have too many pending requests
            loop {
                let pending = { *pending_requests.lock().unwrap() };
                if pending < concurrency.min(MAX_THREADS / 2) { // Limit concurrent requests per endpoint
                    break;
                }
                thread::sleep(Duration::from_millis(1));
            }

            let client = Arc::clone(&self.client);
            let url = url.to_string();
            let auth_header = self.config.auth_header.clone();
            let tenant_header = self.config.tenant_header.clone();
            let success_count = Arc::clone(&success_count);
            let fail_count = Arc::clone(&fail_count);
            let status_counts = Arc::clone(&status_counts);
            let endpoint_latencies = Arc::clone(&endpoint_latencies);
            let scenario_total_requests = Arc::clone(scenario_total_requests);
            let scenario_total_errors = Arc::clone(scenario_total_errors);
            let scenario_latencies = Arc::clone(scenario_latencies);
            let pending_requests = Arc::clone(&pending_requests);
            let completed_requests = Arc::clone(&completed_requests);

            // Increment pending counter
            {
                let mut pending = pending_requests.lock().unwrap();
                *pending += 1;
            }

            self.pool.execute(move || {
                let start_time = Instant::now();
                
                let res = client
                    .get(&url)
                    .header("Authorization", auth_header)
                    .header("tenantId", tenant_header)
                    .send();
                    
                let latency = start_time.elapsed().as_millis() as u64;

                // Update metrics
                endpoint_latencies.lock().unwrap().add_latency(latency);
                scenario_latencies.lock().unwrap().add_latency(latency);

                match res {
                    Ok(response) => {
                        {
                            let mut total = scenario_total_requests.lock().unwrap();
                            *total += 1;
                        }
                        
                        if response.status().is_success() {
                            let mut sc = success_count.lock().unwrap();
                            *sc += 1;
                        } else {
                            let mut fc = fail_count.lock().unwrap();
                            *fc += 1;
                            let mut errors = scenario_total_errors.lock().unwrap();
                            *errors += 1;
                        }
                        let mut sm = status_counts.lock().unwrap();
                        *sm.entry(response.status().as_u16()).or_insert(0) += 1;
                    }
                    Err(e) => {
                        {
                            let mut total = scenario_total_requests.lock().unwrap();
                            *total += 1;
                        }
                        {
                            let mut fc = fail_count.lock().unwrap();
                            *fc += 1;
                        }
                        {
                            let mut errors = scenario_total_errors.lock().unwrap();
                            *errors += 1;
                        }
                        let mut sm = status_counts.lock().unwrap();
                        *sm.entry(0).or_insert(0) += 1;
                        
                        // Log error for debugging
                        if latency > 5000 { // Only log if it took more than 5 seconds
                            eprintln!("Request failed after {}ms: {}", latency, e);
                        }
                    }
                }

                // Decrement pending counter and increment completed
                {
                    let mut pending = pending_requests.lock().unwrap();
                    *pending -= 1;
                }
                {
                    let mut completed = completed_requests.lock().unwrap();
                    *completed += 1;
                }
            });

            // Add small delay between request submissions to prevent overwhelming
            if i > 0 && i % 10 == 0 {
                thread::sleep(request_delay);
            }
        }

        // Wait for all requests to complete with timeout
        let timeout = Duration::from_secs(60); // 60 second timeout
        let start_wait = Instant::now();
        
        loop {
            let completed = { *completed_requests.lock().unwrap() };
            let pending = { *pending_requests.lock().unwrap() };
            
            if completed >= requests || pending == 0 || start_wait.elapsed() > timeout {
                break;
            }
            
            thread::sleep(Duration::from_millis(100));
        }

        // Final wait to ensure all threads finish
        thread::sleep(Duration::from_millis(500));

        let success = *success_count.lock().unwrap();
        let failures = *fail_count.lock().unwrap();
        let total = success + failures;

        let endpoint_lat = endpoint_latencies.lock().unwrap();
        let stats = endpoint_lat.statistical_analysis();
        let p95_with_confidence = endpoint_lat.percentile_with_confidence(95.0);

        let success_rate = if total > 0 {
            (success as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        let mean_latency = endpoint_lat.mean();
        let p95_latency = endpoint_lat.percentile(95.0);
        let p99_latency = endpoint_lat.percentile(99.0);

        let status_map = status_counts.lock().unwrap();
        let mut status_breakdown: Vec<String> = vec![];
        for (status, count) in status_map.iter() {
            status_breakdown.push(format!("{}:{}", status, count));
        }
        let status_string = status_breakdown.join(", ");

        // Show distribution type for debugging
        let distribution_indicator = match stats.distribution_type {
            crate::metrics::DistributionType::Normal => "📊", // Bell curve
            crate::metrics::DistributionType::Skewed => "📈", // Skewed
            crate::metrics::DistributionType::Bimodal => "📉", // Two peaks
            crate::metrics::DistributionType::Unknown => "❓", // Unknown
        };

        println!(
            "Endpoint: {:<60} | Total: {:<4} | Success: {} | Errors: {} | Success Rate: {:.2}% | Mean: {:.2}ms | P95: {}ms (Z:{:.2}) {} | Status: {}",
            endpoint, total, success, failures, success_rate, mean_latency, p95_latency, p95_with_confidence.z_score, distribution_indicator, status_string
        );

        EndpointResult {
            endpoint: endpoint.to_string(),
            total_requests: total,
            successful_requests: success,
            failed_requests: failures,
            success_rate,
            mean_latency,
            p95_latency,
            p99_latency,
            status_codes: status_map.clone(),
        }
    }

    fn print_summary(
        &self,
        total_requests: usize,
        total_errors: usize,
        success_rate: f64,
        rps: f64,
        mean_latency: f64,
        p95_latency: u64,
        duration: f64,
    ) {
        println!("\n==============================");
        println!("Final Overall Summary");
        println!("==============================");
        println!("Total Requests: {}", total_requests);
        println!("Total Errors: {}", total_errors);
        println!("Overall Success Rate: {:.2}%", success_rate);
        println!("Overall RPS: {:.2} req/sec", rps);
        println!("Overall Mean Latency: {:.2}ms", mean_latency);
        println!("Overall P95 Latency: {}ms", p95_latency);
        println!("Overall Duration: {:.2}s", duration);
        println!("==============================");
    }
}
