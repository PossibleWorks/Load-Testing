#[derive(Clone)]
pub struct LatencyMetrics {
    pub latencies: Vec<u64>, // in milliseconds
}

impl LatencyMetrics {
    pub fn new() -> Self {
        LatencyMetrics {
            latencies: Vec::new(),
        }
    }

    pub fn add_latency(&mut self, latency_ms: u64) {
        self.latencies.push(latency_ms);
    }

    pub fn mean(&self) -> f64 {
        if self.latencies.is_empty() {
            return 0.0;
        }
        let sum: u64 = self.latencies.iter().sum();
        sum as f64 / self.latencies.len() as f64
    }

    pub fn percentile(&self, percentile: f64) -> u64 {
        if self.latencies.is_empty() {
            return 0;
        }
        let mut sorted = self.latencies.clone();
        sorted.sort_unstable();
        let index = ((percentile / 100.0) * (sorted.len() - 1) as f64).round() as usize;
        sorted[index.min(sorted.len() - 1)]
    }
}
