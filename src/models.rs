use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Local};

#[derive(Clone)]
pub struct Scenario {
    pub concurrency: usize,
    pub requests: usize,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EndpointResult {
    pub endpoint: String,
    pub total_requests: usize,
    pub successful_requests: usize,
    pub failed_requests: usize,
    pub success_rate: f64,
    pub mean_latency: f64,
    pub p95_latency: u64,
    pub p99_latency: u64,
    pub status_codes: HashMap<u16, usize>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ScenarioResult {
    pub concurrency: usize,
    pub total_requests: usize,
    pub successful_requests: usize,
    pub failed_requests: usize,
    pub success_rate: f64,
    pub rps: f64,
    pub mean_latency: f64,
    pub p95_latency: u64,
    pub p99_latency: u64,
    pub duration_seconds: f64,
    pub endpoints: Vec<EndpointResult>,
}

#[derive(Serialize, Deserialize)]
pub struct LoadTestReport {
    pub test_start_time: DateTime<Local>,
    pub test_end_time: DateTime<Local>,
    pub total_duration_seconds: f64,
    pub overall_requests: usize,
    pub overall_errors: usize,
    pub overall_success_rate: f64,
    pub overall_rps: f64,
    pub overall_mean_latency: f64,
    pub overall_p95_latency: u64,
    pub overall_p99_latency: u64,
    pub scenarios: Vec<ScenarioResult>,
    pub base_url: String,
    pub endpoints_tested: Vec<String>,
}
