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

pub struct LoadTester {
    config: LoadTestConfig,
    client: Arc<Client>,
}

impl LoadTester {
    pub fn new(config: LoadTestConfig) -> Self {
        Self {
            config,
            client: Arc::new(Client::new()),
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

        // Run each endpoint in parallel
        let mut endpoint_threads = vec![];

        for endpoint in endpoints {
            let client = Arc::clone(&self.client);
            let scenario_total_requests = Arc::clone(&scenario_total_requests);
            let scenario_total_errors = Arc::clone(&scenario_total_errors);
            let scenario_latencies = Arc::clone(&scenario_latencies);
            let endpoint = endpoint.to_string();
            let url = format!("{}{}", self.config.base_url, endpoint);
            let auth_header = self.config.auth_header.clone();
            let tenant_header = self.config.tenant_header.clone();
            let concurrency = scenario.concurrency;
            let requests = scenario.requests;

            let handle = thread::spawn(move || {
                Self::run_endpoint_test(
                    &client,
                    &url,
                    &endpoint,
                    &auth_header,
                    &tenant_header,
                    concurrency,
                    requests,
                    &scenario_total_requests,
                    &scenario_total_errors,
                    &scenario_latencies,
                )
            });

            endpoint_threads.push(handle);
        }

        // Wait for all endpoints to finish and collect results
        for handle in endpoint_threads {
            endpoint_results.push(handle.join().unwrap());
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

        println!("Scenario Total Requests: {}", scenario_requests);
        println!("Scenario Total Errors: {}", scenario_errors);
        println!("Scenario Success Rate: {:.2}%", scenario_success_rate);
        println!("Scenario RPS: {:.2} req/sec", scenario_rps);
        println!("Scenario Mean Latency: {:.2}ms", scenario_mean_latency);
        println!("Scenario P95 Latency: {}ms", scenario_p95_latency);
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
        client: &Arc<Client>,
        url: &str,
        endpoint: &str,
        auth_header: &str,
        tenant_header: &str,
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
        let pool = ThreadPool::new(concurrency.min(50));

        for _ in 0..requests {
            let client = Arc::clone(client);
            let url = url.to_string();
            let auth_header = auth_header.to_string();
            let tenant_header = tenant_header.to_string();
            let success_count = Arc::clone(&success_count);
            let fail_count = Arc::clone(&fail_count);
            let status_counts = Arc::clone(&status_counts);
            let endpoint_latencies = Arc::clone(&endpoint_latencies);

            pool.execute(move || {
                let start_time = Instant::now();
                let res = client
                    .get(&url)
                    .header("Authorization", auth_header)
                    .header("tenantId", tenant_header)
                    .send();
                let latency = start_time.elapsed().as_millis() as u64;

                endpoint_latencies.lock().unwrap().add_latency(latency);

                match res {
                    Ok(response) => {
                        if response.status().is_success() {
                            let mut sc = success_count.lock().unwrap();
                            *sc += 1;
                        } else {
                            let mut fc = fail_count.lock().unwrap();
                            *fc += 1;
                        }
                        let mut sm = status_counts.lock().unwrap();
                        *sm.entry(response.status().as_u16()).or_insert(0) += 1;
                    }
                    Err(_) => {
                        let mut fc = fail_count.lock().unwrap();
                        *fc += 1;
                        let mut sm = status_counts.lock().unwrap();
                        *sm.entry(0).or_insert(0) += 1;
                    }
                }

                thread::sleep(Duration::from_millis(1));
            });
        }

        pool.join();

        let success = *success_count.lock().unwrap();
        let failures = *fail_count.lock().unwrap();
        let total = success + failures;

        *scenario_total_requests.lock().unwrap() += total;
        *scenario_total_errors.lock().unwrap() += failures;

        let endpoint_lat = endpoint_latencies.lock().unwrap();
        let mut scenario_lat = scenario_latencies.lock().unwrap();
        for &latency in &endpoint_lat.latencies {
            scenario_lat.add_latency(latency);
        }

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

        println!(
            "Endpoint: {:<60} | Total: {:<4} | Success: {} | Errors: {} | Success Rate: {:.2}% | Mean: {:.2}ms | P95: {}ms | Status: {}",
            endpoint, total, success, failures, success_rate, mean_latency, p95_latency, status_string
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
