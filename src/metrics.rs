#[derive(Clone)]
pub struct LatencyMetrics {
    pub latencies: Vec<u64>, // in milliseconds
}

#[derive(Debug, Clone)]
pub struct StatisticalAnalysis {
    pub mean: f64,
    pub median: f64,
    pub mode: f64,
    pub standard_deviation: f64,
    pub variance: f64,
    pub skewness: f64,
    pub kurtosis: f64,
    pub distribution_type: DistributionType,
}

#[derive(Debug, Clone)]
pub enum DistributionType {
    Normal,      // Bell curve
    Skewed,      // Asymmetric
    Bimodal,     // Two peaks
    Unknown,
}

#[derive(Debug, Clone)]
pub struct PercentileWithConfidence {
    pub value: u64,
    pub confidence_interval_lower: f64,
    pub confidence_interval_upper: f64,
    pub alpha: f64, // Significance level (0.05 for 95%, 0.01 for 99%)
    pub z_score: f64, // Standard score
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

    /// Simple percentile calculation (existing method)
    pub fn percentile(&self, percentile: f64) -> u64 {
        if self.latencies.is_empty() {
            return 0;
        }
        let mut sorted = self.latencies.clone();
        sorted.sort_unstable();
        let index = ((percentile / 100.0) * (sorted.len() - 1) as f64).round() as usize;
        sorted[index.min(sorted.len() - 1)]
    }

    /// Advanced percentile with statistical confidence and alpha values
    pub fn percentile_with_confidence(&self, percentile: f64) -> PercentileWithConfidence {
        let alpha = (100.0 - percentile) / 100.0; // 0.05 for P95, 0.01 for P99
        let z_score = self.calculate_z_score(percentile);
        
        let value = self.percentile(percentile);
        let std_dev = self.standard_deviation();
        let mean = self.mean();
        
        // Calculate confidence interval using normal distribution approximation
        let margin_of_error = z_score * (std_dev / (self.latencies.len() as f64).sqrt());
        
        PercentileWithConfidence {
            value,
            confidence_interval_lower: mean - margin_of_error,
            confidence_interval_upper: mean + margin_of_error,
            alpha,
            z_score,
        }
    }

    /// Full statistical analysis including bell curve characteristics
    pub fn statistical_analysis(&self) -> StatisticalAnalysis {
        if self.latencies.is_empty() {
            return StatisticalAnalysis {
                mean: 0.0,
                median: 0.0,
                mode: 0.0,
                standard_deviation: 0.0,
                variance: 0.0,
                skewness: 0.0,
                kurtosis: 0.0,
                distribution_type: DistributionType::Unknown,
            };
        }

        let mean = self.mean();
        let median = self.median();
        let mode = self.mode();
        let std_dev = self.standard_deviation();
        let variance = self.variance();
        let skewness = self.skewness();
        let kurtosis = self.kurtosis();
        
        // Determine distribution type
        let distribution_type = self.determine_distribution_type(mean, median, skewness, kurtosis);

        StatisticalAnalysis {
            mean,
            median,
            mode,
            standard_deviation: std_dev,
            variance,
            skewness,
            kurtosis,
            distribution_type,
        }
    }

    pub fn median(&self) -> f64 {
        if self.latencies.is_empty() {
            return 0.0;
        }
        let mut sorted = self.latencies.clone();
        sorted.sort_unstable();
        let len = sorted.len();
        if len % 2 == 0 {
            (sorted[len / 2 - 1] + sorted[len / 2]) as f64 / 2.0
        } else {
            sorted[len / 2] as f64
        }
    }

    pub fn mode(&self) -> f64 {
        if self.latencies.is_empty() {
            return 0.0;
        }
        
        use std::collections::HashMap;
        let mut frequency = HashMap::new();
        
        for &latency in &self.latencies {
            *frequency.entry(latency).or_insert(0) += 1;
        }
        
        frequency.into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(latency, _)| latency as f64)
            .unwrap_or(0.0)
    }

    pub fn standard_deviation(&self) -> f64 {
        self.variance().sqrt()
    }

    pub fn variance(&self) -> f64 {
        if self.latencies.len() < 2 {
            return 0.0;
        }
        
        let mean = self.mean();
        let sum_squared_diffs: f64 = self.latencies.iter()
            .map(|&x| {
                let diff = x as f64 - mean;
                diff * diff
            })
            .sum();
        
        sum_squared_diffs / (self.latencies.len() - 1) as f64
    }

    pub fn skewness(&self) -> f64 {
        if self.latencies.len() < 3 {
            return 0.0;
        }

        let mean = self.mean();
        let std_dev = self.standard_deviation();
        let n = self.latencies.len() as f64;

        let sum_cubed_z_scores: f64 = self.latencies.iter()
            .map(|&x| {
                let z = (x as f64 - mean) / std_dev;
                z * z * z
            })
            .sum();

        (n / ((n - 1.0) * (n - 2.0))) * sum_cubed_z_scores
    }

    pub fn kurtosis(&self) -> f64 {
        if self.latencies.len() < 4 {
            return 0.0;
        }

        let mean = self.mean();
        let std_dev = self.standard_deviation();
        let n = self.latencies.len() as f64;

        let sum_fourth_z_scores: f64 = self.latencies.iter()
            .map(|&x| {
                let z = (x as f64 - mean) / std_dev;
                z * z * z * z
            })
            .sum();

        let numerator = (n * (n + 1.0)) / ((n - 1.0) * (n - 2.0) * (n - 3.0));
        let correction = (3.0 * (n - 1.0) * (n - 1.0)) / ((n - 2.0) * (n - 3.0));
        
        numerator * sum_fourth_z_scores - correction
    }

    /// Calculate Z-score for given percentile (critical values for normal distribution)
    fn calculate_z_score(&self, percentile: f64) -> f64 {
        match percentile as u8 {
            90 => 1.282,  // P90
            95 => 1.645,  // P95
            99 => 2.326,  // P99
            _ => {
                // Approximate calculation for other percentiles
                let p = percentile / 100.0;
                // Box-Muller transformation approximation
                if p > 0.5 {
                    let t = (-2.0 * (1.0 - p).ln()).sqrt();
                    t - (2.515517 + 0.802853 * t + 0.010328 * t * t) / 
                        (1.0 + 1.432788 * t + 0.189269 * t * t + 0.001308 * t * t * t)
                } else {
                    let t = (-2.0 * p.ln()).sqrt();
                    -1.0 * (t - (2.515517 + 0.802853 * t + 0.010328 * t * t) / 
                            (1.0 + 1.432788 * t + 0.189269 * t * t + 0.001308 * t * t * t))
                }
            }
        }
    }

    /// Determine if data follows normal distribution (bell curve)
    fn determine_distribution_type(&self, mean: f64, median: f64, skewness: f64, kurtosis: f64) -> DistributionType {
        let mean_median_ratio = (mean - median).abs() / mean.max(1.0);
        
        // Check for normal distribution (bell curve)
        if skewness.abs() < 0.5 && kurtosis.abs() < 1.0 && mean_median_ratio < 0.1 {
            DistributionType::Normal
        } else if skewness.abs() > 1.0 {
            DistributionType::Skewed
        } else if self.has_multiple_modes() {
            DistributionType::Bimodal
        } else {
            DistributionType::Unknown
        }
    }

    /// Check for multiple modes (bimodal distribution)
    fn has_multiple_modes(&self) -> bool {
        if self.latencies.len() < 10 {
            return false;
        }

        use std::collections::HashMap;
        let mut frequency = HashMap::new();
        
        for &latency in &self.latencies {
            *frequency.entry(latency).or_insert(0) += 1;
        }
        
        let mut frequencies: Vec<_> = frequency.values().collect();
        frequencies.sort_unstable();
        frequencies.reverse();
        
        // Check if there are at least 2 modes with similar high frequencies
        frequencies.len() >= 2 && frequencies[0] > &1 && frequencies[1] > &1 &&
        (frequencies[0] - frequencies[1]) <= 2
    }

    /// Generate a simple histogram for visualizing distribution
    pub fn histogram(&self, bins: usize) -> Vec<(u64, usize)> {
        if self.latencies.is_empty() {
            return Vec::new();
        }

        let min = *self.latencies.iter().min().unwrap();
        let max = *self.latencies.iter().max().unwrap();
        let bin_size = if max > min { (max - min) / bins as u64 } else { 1 };

        let mut histogram = vec![0; bins];
        
        for &latency in &self.latencies {
            let bin_index = if bin_size > 0 {
                ((latency - min) / bin_size).min(bins as u64 - 1) as usize
            } else {
                0
            };
            histogram[bin_index] += 1;
        }

        histogram.into_iter()
            .enumerate()
            .map(|(i, count)| (min + i as u64 * bin_size, count))
            .collect()
    }
}
