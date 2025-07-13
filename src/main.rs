use reqwest::blocking::Client;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use threadpool::ThreadPool;

#[derive(Clone)]
struct Scenario {
    concurrency: usize,
    requests: usize,
}

#[derive(Clone)]
struct LatencyMetrics {
    latencies: Vec<u64>, // in milliseconds
}

impl LatencyMetrics {
    fn new() -> Self {
        LatencyMetrics {
            latencies: Vec::new(),
        }
    }

    fn add_latency(&mut self, latency_ms: u64) {
        self.latencies.push(latency_ms);
    }

    fn mean(&self) -> f64 {
        if self.latencies.is_empty() {
            return 0.0;
        }
        let sum: u64 = self.latencies.iter().sum();
        sum as f64 / self.latencies.len() as f64
    }

    fn percentile(&self, percentile: f64) -> u64 {
        if self.latencies.is_empty() {
            return 0;
        }
        let mut sorted = self.latencies.clone();
        sorted.sort_unstable();
        let index = ((percentile / 100.0) * (sorted.len() - 1) as f64).round() as usize;
        sorted[index.min(sorted.len() - 1)]
    }
}

fn main() {
    // ==============================
    // 🔗 Base Setup
    // ==============================
    const BASE_URL: &str = "https://bev3-dev.lykkebook.com/api";
    const AUTH_HEADER: &str = "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3N1ZXIiOiJwb3NzaWJsZXdvcmtzLmNvbSIsImF1ZGllbmNlIjoicG9zc2libGV3b3Jrcy5jb20iLCJmaXJzdE5hbWUiOiJjZW8iLCJsYXN0TmFtZSI6ImNsaWVudF9xYSIsImVtYWlsIjoiY2VvQHB3LmNvbSIsImlkIjoiODQ2OTZjYzktMzViNy00NGYzLWI4YTctMWZhZDI2NmMyYTkwIiwidGVuYW50VXNlcklkIjoiMGY3NTY3MWUtYTAyNC00M2JjLTgyYWEtMWY0NTRmMzlmZmQ1Iiwic3ViIjoiMGY3NTY3MWUtYTAyNC00M2JjLTgyYWEtMWY0NTRmMzlmZmQ1IiwiaWF0IjoxNzUyMzc4ODMxMDI0LCJhdXRoX3RpbWUiOjE3NTIzNzg4MzEwMjQsInJvbGUiOiIyIiwidGVuYW50aWQiOiJhYzAwNzBhMS1jMjdlLTQ4ZWYtYWFmMi00MzAzYmRmY2UyYzUiLCJzdGF0dXMiOiJhY3RpdmUiLCJleHAiOjE3NTIzNzg5MTc0MjR9.hRhFxHtVcKQZZVDKb4VpXVZUj2wnnWQc0Dn0eFc9NWQ";
    const TENANT_HEADER: &str = "ac0070a1-c27e-48ef-aaf2-4303bdfce2c5";

    // Example IDs
    let user_id = "0f75671e-a024-43bc-82aa-1f454f39ffd5";
    let period_id = "5b39887b-6659-4d87-975f-508917131ea3";
    let cycle_id = "a915cb6e-0974-4fb8-9553-81f04ce7ca45";

    // ==============================
    // 📌 Endpoints
    // ==============================
    let endpoints = vec![
        format!("/auth/get-tenants"),
        format!("/user/me"),
        format!("/user/active-and-inactive-users"),
        format!("/user/get-user-skills/{}", user_id),
        format!("/user/team/{}", user_id),
        format!("/user/chat-users/"),
        format!("/user/all-users"),
        format!(
            "/user/user-progress-details/{}/{}/{}",
            user_id, period_id, cycle_id
        ),
        format!("/goals/user-goals/{}/{}", period_id, user_id),
        format!("/goals/manager-reportee/{}/{}", period_id, user_id),
        format!("/goals/eligible-reportees-for-cascade/{}", period_id),
        format!("/goals/get-goal-categories"),
        format!("/goals/get-objectives-akrs-and-quadrants/{}", cycle_id),
        format!("/goals/detailed-goals/{}/{}", period_id, user_id),
        format!("/goals/cascade-json/{}/{}", user_id, period_id),
        format!("/periods/previous-current-next"),
        format!("/periods/{}/subperiods", period_id),
        format!("/cycles/"),
    ];

    let scenarios = vec![Scenario {
        concurrency: 50,
        requests: 50,
    }];

    let client = Arc::new(Client::new());

    let mut overall_total_requests = 0;
    let mut overall_total_errors = 0;
    let mut overall_latencies = LatencyMetrics::new();
    let overall_start_time = Instant::now();

    for scenario in scenarios {
        println!(
            "\n🚀 Scenario: Concurrency {}, Requests {}",
            scenario.concurrency, scenario.requests
        );

        let scenario_start_time = Instant::now();
        let scenario_total_requests = Arc::new(Mutex::new(0));
        let scenario_total_errors = Arc::new(Mutex::new(0));
        let scenario_latencies = Arc::new(Mutex::new(LatencyMetrics::new()));

        // 🔑 One thread per endpoint (they run in parallel)
        let mut endpoint_threads = vec![];

        for endpoint in endpoints.clone() {
            let client = Arc::clone(&client);
            let scenario_total_requests = Arc::clone(&scenario_total_requests);
            let scenario_total_errors = Arc::clone(&scenario_total_errors);
            let scenario_latencies = Arc::clone(&scenario_latencies);
            let endpoint = endpoint.to_string();
            let url = format!("{}{}", BASE_URL, endpoint);

            let handle = thread::spawn(move || {
                let success_count = Arc::new(Mutex::new(0));
                let fail_count = Arc::new(Mutex::new(0));
                let endpoint_latencies = Arc::new(Mutex::new(LatencyMetrics::new()));

                let status_counts = Arc::new(Mutex::new(HashMap::new()));
                let pool = ThreadPool::new(scenario.concurrency.min(50));

                for _ in 0..scenario.requests {
                    let client = Arc::clone(&client);
                    let url = url.clone();
                    let success_count = Arc::clone(&success_count);
                    let fail_count = Arc::clone(&fail_count);
                    let status_counts = Arc::clone(&status_counts);
                    let endpoint_latencies = Arc::clone(&endpoint_latencies);

                    pool.execute(move || {
                        let start_time = Instant::now();
                        let res = client
                            .get(&url)
                            .header("Authorization", AUTH_HEADER)
                            .header("tenantId", TENANT_HEADER)
                            .send();
                        let latency = start_time.elapsed().as_millis() as u64;

                        // Add latency to metrics
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
                                *sm.entry(0).or_insert(0) += 1; // Use 0 for transport errors
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

                // Merge endpoint latencies into scenario latencies
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

                let status_map = status_counts.lock().unwrap();
                let mut status_breakdown: Vec<String> = vec![];
                for (status, count) in status_map.iter() {
                    status_breakdown.push(format!("{}:{}", status, count));
                }
                let status_string = status_breakdown.join(", ");

                println!(
    "🔗 Endpoint: {:<60} | Total: {:<4} | ✅ {} | ❌ {} | Success Rate: {:.2}% | Mean: {:.2}ms | P95: {}ms | Status: {}",
    endpoint, total, success, failures, success_rate, mean_latency, p95_latency, status_string
);
            });

            endpoint_threads.push(handle);
        }

        // 🔑 Wait for all endpoints to finish
        for handle in endpoint_threads {
            handle.join().unwrap();
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

        // Add scenario latencies to overall latencies
        for &latency in &scenario_lat.latencies {
            overall_latencies.add_latency(latency);
        }

        println!("📦 Scenario Total Requests: {}", scenario_requests);
        println!("❌ Scenario Total Errors: {}", scenario_errors);
        println!("✅ Scenario Success Rate: {:.2}%", scenario_success_rate);
        println!("🚀 Scenario RPS: {:.2} req/sec", scenario_rps);
        println!("📊 Scenario Mean Latency: {:.2}ms", scenario_mean_latency);
        println!("📈 Scenario P95 Latency: {}ms", scenario_p95_latency);
        println!("⏱️  Scenario Duration: {:.2}s", scenario_duration.as_secs_f64());

        overall_total_requests += scenario_requests;
        overall_total_errors += scenario_errors;
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

    println!("\n==============================");
    println!("📝 Final Overall Summary");
    println!("==============================");
    println!("📦 Total Requests: {}", overall_total_requests);
    println!("❌ Total Errors: {}", overall_total_errors);
    println!("✅ Overall Success Rate: {:.2}%", overall_success_rate);
    println!("🚀 Overall RPS: {:.2} req/sec", overall_rps);
    println!("📊 Overall Mean Latency: {:.2}ms", overall_mean_latency);
    println!("📈 Overall P95 Latency: {}ms", overall_p95_latency);
    println!("⏱️  Overall Duration: {:.2}s", overall_duration.as_secs_f64());
    println!("==============================");
}