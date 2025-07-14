use crate::models::LoadTestReport;
use std::fs::File;
use std::io::Write;

impl LoadTestReport {
    pub fn generate_html(&self) -> String {
        let chart_data = self.generate_chart_data();
        
        format!(r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Load Test Report</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; line-height: 1.6; }}
        .header {{ text-align: center; border-bottom: 2px solid #333; padding-bottom: 20px; }}
        .summary-table {{ border-collapse: collapse; width: 100%; margin: 20px 0; }}
        .summary-table th, .summary-table td {{ border: 1px solid #ddd; padding: 12px; text-align: left; }}
        .summary-table th {{ background-color: #f2f2f2; }}
        .section {{ margin: 30px 0; }}
        .endpoint-table {{ border-collapse: collapse; width: 100%; font-size: 12px; }}
        .endpoint-table th, .endpoint-table td {{ border: 1px solid #ddd; padding: 8px; text-align: left; }}
        .endpoint-table th {{ background-color: #f2f2f2; }}
        .metrics {{ display: flex; justify-content: space-around; margin: 20px 0; }}
        .metric-box {{ border: 1px solid #ddd; padding: 15px; text-align: center; background-color: #f9f9f9; }}
        .chart-container {{ border: 1px solid #ddd; margin: 20px 0; padding: 20px; background-color: #f9f9f9; }}
    </style>
</head>
<body>
    <div class="header">
        <h1>Load Test Report</h1>
        <p>Generated on: {}</p>
        <p>Test Duration: {:.2} seconds</p>
    </div>

    <div class="section">
        <h2>1. Test Objectives</h2>
        <p>This load test was conducted to evaluate the performance and scalability of the API endpoints under various load conditions. The test aimed to:</p>
        <ul>
            <li>Determine the maximum sustainable request rate</li>
            <li>Measure response times under different concurrency levels</li>
            <li>Identify potential bottlenecks and failure points</li>
            <li>Validate system stability under stress conditions</li>
        </ul>
    </div>

    <div class="section">
        <h2>2. Test Configuration</h2>
        <table class="summary-table">
            <tr><th>Parameter</th><th>Value</th></tr>
            <tr><td>Base URL</td><td>{}</td></tr>
            <tr><td>Total Endpoints Tested</td><td>{}</td></tr>
            <tr><td>Test Start Time</td><td>{}</td></tr>
            <tr><td>Test End Time</td><td>{}</td></tr>
        </table>
    </div>

    <div class="section">
        <h2>3. Overall Results Summary</h2>
        <div class="metrics">
            <div class="metric-box">
                <h3>Total Requests</h3>
                <h2>{}</h2>
            </div>
            <div class="metric-box">
                <h3>Success Rate</h3>
                <h2>{:.2}%</h2>
            </div>
            <div class="metric-box">
                <h3>Average RPS</h3>
                <h2>{:.2}</h2>
            </div>
            <div class="metric-box">
                <h3>Mean Latency</h3>
                <h2>{:.2}ms</h2>
            </div>
        </div>

        <table class="summary-table">
            <tr><th>Metric</th><th>Value</th></tr>
            <tr><td>Total Requests</td><td>{}</td></tr>
            <tr><td>Successful Requests</td><td>{}</td></tr>
            <tr><td>Failed Requests</td><td>{}</td></tr>
            <tr><td>Success Rate</td><td>{:.2}%</td></tr>
            <tr><td>Average RPS</td><td>{:.2} req/sec</td></tr>
            <tr><td>Mean Latency</td><td>{:.2}ms</td></tr>
            <tr><td>95th Percentile Latency</td><td>{}ms</td></tr>
            <tr><td>99th Percentile Latency</td><td>{}ms</td></tr>
        </table>
    </div>

    <div class="section">
        <h2>4. Scaling Analysis & Comparison</h2>
        {}
        
        <h3>Performance Trends</h3>
        <table class="summary-table">
            <tr>
                <th>Concurrency</th>
                <th>Total Requests</th>
                <th>Success Rate (%)</th>
                <th>RPS</th>
                <th>Mean Latency (ms)</th>
                <th>P95 Latency (ms)</th>
                <th>P99 Latency (ms)</th>
                <th>Duration (s)</th>
                <th>Performance Score</th>
            </tr>
            {}
        </table>
        
        <h3>Key Performance Insights</h3>
        <div class="metrics">
            <div class="metric-box">
                <h4>Best Performing Concurrency</h4>
                <h3>{}</h3>
                <p>Highest RPS with good latency</p>
            </div>
            <div class="metric-box">
                <h4>Breaking Point</h4>
                <h3>{}</h3>
                <p>Where performance degrades</p>
            </div>
            <div class="metric-box">
                <h4>Optimal Load Range</h4>
                <h3>{} - {}</h3>
                <p>Recommended operating range</p>
            </div>
            <div class="metric-box">
                <h4>Scalability Factor</h4>
                <h3>{:.1}x</h3>
                <p>Performance improvement ratio</p>
            </div>
        </div>
    </div>

    <div class="section">
        <h2>5. Detailed Scenario Results</h2>
        {}
    </div>

    <div class="section">
        <h2>6. Performance Charts</h2>
        
        <div class="chart-container">
            <h3>Scaling Performance Overview (Latency vs RPS)</h3>
            <canvas id="scalingChart" width="800" height="400"></canvas>
        </div>
        
        <div class="chart-container">
            <h3>Latency Scaling by Concurrency</h3>
            <canvas id="latencyScalingChart" width="800" height="400"></canvas>
        </div>
        
        <div class="chart-container">
            <h3>Throughput Scaling (RPS by Concurrency)</h3>
            <canvas id="throughputChart" width="800" height="400"></canvas>
        </div>
        
        <div class="chart-container">
            <h3>Success Rate by Concurrency Level</h3>
            <canvas id="successRateChart" width="800" height="400"></canvas>
        </div>
        
        <div class="chart-container">
            <h3>Endpoint Response Times (Best Scenario)</h3>
            <canvas id="endpointLatencyChart" width="800" height="400"></canvas>
        </div>
        
        <div class="chart-container">
            <h3>Overall Test Results</h3>
            <canvas id="overallChart" width="400" height="400"></canvas>
        </div>
    </div>
    
    <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
    <script>
        {}
    </script>
</body>
</html>"#,
            self.test_end_time.format("%Y-%m-%d %H:%M:%S"),
            self.total_duration_seconds,
            self.base_url,
            self.endpoints_tested.len(),
            self.test_start_time.format("%Y-%m-%d %H:%M:%S"),
            self.test_end_time.format("%Y-%m-%d %H:%M:%S"),
            self.overall_requests,
            self.overall_success_rate,
            self.overall_rps,
            self.overall_mean_latency,
            self.overall_requests,
            self.overall_requests - self.overall_errors,
            self.overall_errors,
            self.overall_success_rate,
            self.overall_rps,
            self.overall_mean_latency,
            self.overall_p95_latency,
            self.overall_p99_latency,
            self.generate_scaling_summary(),
            self.generate_scaling_comparison_table(),
            self.get_best_performing_concurrency(),
            self.get_breaking_point(),
            {
                let (start, _end) = self.get_optimal_range();
                format!("{}", start)
            },
            {
                let (_start, end) = self.get_optimal_range();
                format!("{}", end)
            },
            self.get_scalability_factor(),
            self.generate_scenario_html(),
            chart_data
        )
    }

    fn generate_scenario_html(&self) -> String {
        let mut html = String::new();
        
        for (index, scenario) in self.scenarios.iter().enumerate() {
            html.push_str(&format!(r#"
        <h3>Scenario {} - Concurrency: {}</h3>
        <table class="summary-table">
            <tr><th>Metric</th><th>Value</th></tr>
            <tr><td>Concurrency</td><td>{}</td></tr>
            <tr><td>Total Requests</td><td>{}</td></tr>
            <tr><td>Success Rate</td><td>{:.2}%</td></tr>
            <tr><td>RPS</td><td>{:.2}</td></tr>
            <tr><td>Mean Latency</td><td>{:.2}ms</td></tr>
            <tr><td>P95 Latency</td><td>{}ms</td></tr>
            <tr><td>Duration</td><td>{:.2}s</td></tr>
        </table>

        <h4>Endpoint Details:</h4>
        <table class="endpoint-table">
            <tr>
                <th>Endpoint</th>
                <th>Requests</th>
                <th>Success</th>
                <th>Errors</th>
                <th>Success Rate</th>
                <th>Mean Latency</th>
                <th>P95 Latency</th>
            </tr>
            {}
        </table>
        "#,
                index + 1,
                scenario.concurrency,
                scenario.concurrency,
                scenario.total_requests,
                scenario.success_rate,
                scenario.rps,
                scenario.mean_latency,
                scenario.p95_latency,
                scenario.duration_seconds,
                scenario.endpoints.iter().map(|ep| {
                    format!(r#"
            <tr>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>{:.2}%</td>
                <td>{:.2}ms</td>
                <td>{}ms</td>
            </tr>"#,
                        ep.endpoint,
                        ep.total_requests,
                        ep.successful_requests,
                        ep.failed_requests,
                        ep.success_rate,
                        ep.mean_latency,
                        ep.p95_latency
                    )
                }).collect::<Vec<_>>().join("")
            ));
        }
        
        html
    }

    pub fn generate_markdown(&self) -> String {
        format!(r#"# Load Test Report - Scaling Analysis

*Generated on: {}*  
*Test Duration: {:.2} seconds*

## 1. Test Objectives

This load test was conducted to evaluate the performance and scalability of the API endpoints under various load conditions. The test aimed to:

- Determine the maximum sustainable request rate
- Measure response times under different concurrency levels  
- Identify potential bottlenecks and failure points
- Validate system stability under stress conditions
- Analyze scaling characteristics from {} to {} concurrent connections

## 2. Test Configuration

| Parameter | Value |
|-----------|-------|
| Base URL | {} |
| Total Endpoints Tested | {} |
| Scaling Levels | {} scenarios |
| Concurrency Range | {} - {} connections |
| Total Requests | {} |
| Test Start Time | {} |
| Test End Time | {} |

## 3. Scaling Analysis & Performance Summary

{}

### Performance Trends

| Concurrency | Total Requests | Success Rate (%) | RPS | Mean Latency (ms) | P95 Latency (ms) | Duration (s) | Performance Score |
|-------------|----------------|------------------|-----|-------------------|------------------|--------------|-------------------|
{}

### Key Performance Insights

- **Best Performing Concurrency:** {} connections
- **Breaking Point:** {} connections  
- **Optimal Load Range:** {} - {} connections
- **Scalability Factor:** {:.1}x improvement from baseline to peak
- **Linear Scaling:** {}

## 4. Overall Results Summary

### Key Metrics
- **Total Requests:** {}
- **Success Rate:** {:.2}%
- **Average RPS:** {:.2}
- **Mean Latency:** {:.2}ms

### Detailed Results

| Metric | Value |
|--------|-------|
| Total Requests | {} |
| Successful Requests | {} |
| Failed Requests | {} |
| Success Rate | {:.2}% |
| Average RPS | {:.2} req/sec |
| Mean Latency | {:.2}ms |
| 95th Percentile Latency | {}ms |
| 99th Percentile Latency | {}ms |

## 5. Detailed Scenario Results

{}

## 6. Recommendations

Based on the scaling analysis:

1. **Optimal Operating Range:** {} - {} concurrent connections provide the best balance of throughput and latency
2. **Performance Degradation:** Monitor closely beyond {} concurrent connections
3. **Resource Planning:** System shows {}x scaling capability from baseline to peak performance
4. **Bottleneck Analysis:** {}

"#,
            self.test_end_time.format("%Y-%m-%d %H:%M:%S"),
            self.total_duration_seconds,
            self.scenarios.first().map(|s| s.concurrency).unwrap_or(0),
            self.scenarios.last().map(|s| s.concurrency).unwrap_or(0),
            self.base_url,
            self.endpoints_tested.len(),
            self.scenarios.len(),
            self.scenarios.first().map(|s| s.concurrency).unwrap_or(0),
            self.scenarios.last().map(|s| s.concurrency).unwrap_or(0),
            self.overall_requests,
            self.test_start_time.format("%Y-%m-%d %H:%M:%S"),
            self.test_end_time.format("%Y-%m-%d %H:%M:%S"),
            self.generate_scaling_summary_markdown(),
            self.generate_scaling_table_markdown(),
            self.get_best_performing_concurrency(),
            self.get_breaking_point(),
            {
                let (start, end) = self.get_optimal_range();
                start
            },
            {
                let (start, end) = self.get_optimal_range();
                end
            },
            self.get_scalability_factor(),
            if self.is_linear_scaling() { "Yes" } else { "No" },
            self.overall_requests,
            self.overall_success_rate,
            self.overall_rps,
            self.overall_mean_latency,
            self.overall_requests,
            self.overall_requests - self.overall_errors,
            self.overall_errors,
            self.overall_success_rate,
            self.overall_rps,
            self.overall_mean_latency,
            self.overall_p95_latency,
            self.overall_p99_latency,
            self.generate_scenario_markdown(),
            {
                let (start, _end) = self.get_optimal_range();
                start
            },
            {
                let (_start, end) = self.get_optimal_range();
                end
            },
            self.get_scalability_factor(),
            self.get_bottleneck_analysis(),
            // Add the missing method call for the recommendations section
            self.get_recommendations()
        )
    }

    fn generate_scenario_markdown(&self) -> String {
        let mut markdown = String::new();
        
        for (index, scenario) in self.scenarios.iter().enumerate() {
            markdown.push_str(&format!(r#"
### Scenario {} - Concurrency: {}

| Metric | Value |
|--------|-------|
| Concurrency | {} |
| Total Requests | {} |
| Success Rate | {:.2}% |
| RPS | {:.2} |
| Mean Latency | {:.2}ms |
| P95 Latency | {}ms |
| Duration | {:.2}s |

#### Endpoint Details:

| Endpoint | Requests | Success | Errors | Success Rate | Mean Latency | P95 Latency |
|----------|----------|---------|--------|--------------|--------------|-------------|
| {} |
"#,
                index + 1,
                scenario.concurrency,
                scenario.concurrency,
                scenario.total_requests,
                scenario.success_rate,
                scenario.rps,
                scenario.mean_latency,
                scenario.p95_latency,
                scenario.duration_seconds,
                scenario.endpoints.iter().map(|ep| {
                    format!("| {} | {} | {} | {} | {:.2}% | {:.2}ms | {}ms |",
                        ep.endpoint,
                        ep.total_requests,
                        ep.successful_requests,
                        ep.failed_requests,
                        ep.success_rate,
                        ep.mean_latency,
                        ep.p95_latency
                    )
                }).collect::<Vec<_>>().join("\n")
            ));
        }
        
        markdown
    }

    fn generate_chart_data(&self) -> String {
        // Extract scaling data for charts
        let concurrency_labels: Vec<String> = self.scenarios.iter()
            .map(|s| format!("\"{}\"", s.concurrency))
            .collect();
        
        let mean_latencies: Vec<String> = self.scenarios.iter()
            .map(|s| s.mean_latency.to_string())
            .collect();
            
        let p95_latencies: Vec<String> = self.scenarios.iter()
            .map(|s| s.p95_latency.to_string())
            .collect();
            
        let rps_values: Vec<String> = self.scenarios.iter()
            .map(|s| s.rps.to_string())
            .collect();
            
        let success_rates: Vec<String> = self.scenarios.iter()
            .map(|s| s.success_rate.to_string())
            .collect();
        
        // Get endpoint data from the best performing scenario
        let best_scenario = self.scenarios.iter()
            .max_by(|a, b| a.rps.partial_cmp(&b.rps).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(&self.scenarios[0]);
        
        let endpoint_data = best_scenario.endpoints.iter()
            .map(|ep| (
                format!("\"{}\"", ep.endpoint.replace('"', "\\\"")[..ep.endpoint.len().min(30)].to_string()),
                ep.mean_latency.to_string(),
                ep.success_rate.to_string()
            ))
            .collect::<Vec<_>>();
        
        let endpoint_labels: Vec<String> = endpoint_data.iter().map(|(label, _, _)| label.clone()).collect();
        let endpoint_latencies: Vec<String> = endpoint_data.iter().map(|(_, latency, _)| latency.clone()).collect();

        format!(r#"
        // Chart.js configuration and data
        const chartColors = {{
            primary: '#3498db',
            success: '#2ecc71',
            warning: '#f39c12',
            danger: '#e74c3c',
            info: '#9b59b6',
            secondary: '#95a5a6'
        }};

        // Scaling Performance Overview (Scatter plot: RPS vs Latency)
        const scalingCtx = document.getElementById('scalingChart').getContext('2d');
        new Chart(scalingCtx, {{
            type: 'scatter',
            data: {{
                datasets: [{{
                    label: 'Performance Points',
                    data: [{}],
                    backgroundColor: chartColors.primary,
                    borderColor: chartColors.primary,
                    pointRadius: 8
                }}]
            }},
            options: {{
                responsive: true,
                plugins: {{
                    title: {{
                        display: true,
                        text: 'Scaling Performance: RPS vs Latency by Concurrency'
                    }},
                    tooltip: {{
                        callbacks: {{
                            label: function(context) {{
                                const concurrency = [{}][context.dataIndex];
                                return `Concurrency ${{concurrency}}: ${{context.parsed.x}} RPS, ${{context.parsed.y}}ms latency`;
                            }}
                        }}
                    }}
                }},
                scales: {{
                    x: {{
                        title: {{
                            display: true,
                            text: 'Requests Per Second (RPS)'
                        }}
                    }},
                    y: {{
                        title: {{
                            display: true,
                            text: 'Mean Latency (ms)'
                        }}
                    }}
                }}
            }}
        }});

        // Latency Scaling Chart
        const latencyScalingCtx = document.getElementById('latencyScalingChart').getContext('2d');
        new Chart(latencyScalingCtx, {{
            type: 'line',
            data: {{
                labels: [{}],
                datasets: [{{
                    label: 'Mean Latency (ms)',
                    data: [{}],
                    borderColor: chartColors.primary,
                    backgroundColor: chartColors.primary + '20',
                    tension: 0.1,
                    fill: false
                }}, {{
                    label: 'P95 Latency (ms)',
                    data: [{}],
                    borderColor: chartColors.warning,
                    backgroundColor: chartColors.warning + '20',
                    tension: 0.1,
                    fill: false
                }}]
            }},
            options: {{
                responsive: true,
                plugins: {{
                    title: {{
                        display: true,
                        text: 'Response Time Scaling by Concurrency Level'
                    }}
                }},
                scales: {{
                    y: {{
                        beginAtZero: true,
                        title: {{
                            display: true,
                            text: 'Latency (milliseconds)'
                        }}
                    }},
                    x: {{
                        title: {{
                            display: true,
                            text: 'Concurrent Connections'
                        }}
                    }}
                }}
            }}
        }});

        // Throughput Chart
        const throughputCtx = document.getElementById('throughputChart').getContext('2d');
        new Chart(throughputCtx, {{
            type: 'bar',
            data: {{
                labels: [{}],
                datasets: [{{
                    label: 'Requests Per Second',
                    data: [{}],
                    backgroundColor: chartColors.success,
                    borderColor: chartColors.success,
                    borderWidth: 1
                }}]
            }},
            options: {{
                responsive: true,
                plugins: {{
                    title: {{
                        display: true,
                        text: 'Throughput Scaling (RPS by Concurrency)'
                    }}
                }},
                scales: {{
                    y: {{
                        beginAtZero: true,
                        title: {{
                            display: true,
                            text: 'Requests Per Second'
                        }}
                    }},
                    x: {{
                        title: {{
                            display: true,
                            text: 'Concurrent Connections'
                        }}
                    }}
                }}
            }}
        }});

        // Success Rate Chart
        const successRateCtx = document.getElementById('successRateChart').getContext('2d');
        new Chart(successRateCtx, {{
            type: 'line',
            data: {{
                labels: [{}],
                datasets: [{{
                    label: 'Success Rate (%)',
                    data: [{}],
                    borderColor: chartColors.success,
                    backgroundColor: chartColors.success + '20',
                    tension: 0.1,
                    fill: true
                }}]
            }},
            options: {{
                responsive: true,
                plugins: {{
                    title: {{
                        display: true,
                        text: 'Success Rate by Concurrency Level'
                    }}
                }},
                scales: {{
                    y: {{
                        min: 0,
                        max: 100,
                        title: {{
                            display: true,
                            text: 'Success Rate (%)'
                        }}
                    }},
                    x: {{
                        title: {{
                            display: true,
                            text: 'Concurrent Connections'
                        }}
                    }}
                }}
            }}
        }});

        // Endpoint Response Times Chart
        const endpointLatencyCtx = document.getElementById('endpointLatencyChart').getContext('2d');
        new Chart(endpointLatencyCtx, {{
            type: 'bar',
            data: {{
                labels: [{}],
                datasets: [{{
                    label: 'Mean Latency (ms)',
                    data: [{}],
                    backgroundColor: chartColors.primary,
                    borderColor: chartColors.primary,
                    borderWidth: 1
                }}]
            }},
            options: {{
                responsive: true,
                plugins: {{
                    title: {{
                        display: true,
                        text: 'Endpoint Response Times (Best Performing Scenario: {} concurrent)'
                    }}
                }},
                scales: {{
                    y: {{
                        beginAtZero: true,
                        title: {{
                            display: true,
                            text: 'Mean Latency (milliseconds)'
                        }}
                    }},
                    x: {{
                        title: {{
                            display: true,
                            text: 'API Endpoints'
                        }},
                        ticks: {{
                            maxRotation: 45,
                            minRotation: 45
                        }}
                    }}
                }}
            }}
        }});

        // Overall Results Chart
        const overallCtx = document.getElementById('overallChart').getContext('2d');
        new Chart(overallCtx, {{
            type: 'doughnut',
            data: {{
                labels: ['Successful Requests', 'Failed Requests'],
                datasets: [{{
                    data: [{}, {}],
                    backgroundColor: [chartColors.success, chartColors.danger],
                    borderWidth: 2
                }}]
            }},
            options: {{
                responsive: true,
                plugins: {{
                    title: {{
                        display: true,
                        text: 'Overall Test Results Summary'
                    }},
                    legend: {{
                        position: 'bottom'
                    }}
                }}
            }}
        }});
        "#,
            // Scaling chart data (RPS vs Latency scatter points)
            self.scenarios.iter().map(|s| format!("{{x: {}, y: {}}}", s.rps, s.mean_latency)).collect::<Vec<_>>().join(", "),
            self.scenarios.iter().map(|s| s.concurrency.to_string()).collect::<Vec<_>>().join(", "),
            
            // Latency scaling chart
            concurrency_labels.join(", "),
            mean_latencies.join(", "),
            p95_latencies.join(", "),
            
            // Throughput chart
            concurrency_labels.join(", "),
            rps_values.join(", "),
            
            // Success rate chart
            concurrency_labels.join(", "),
            success_rates.join(", "),
            
            // Endpoint chart
            endpoint_labels.join(", "),
            endpoint_latencies.join(", "),
            best_scenario.concurrency,
            
            // Overall results
            self.overall_requests - self.overall_errors,
            self.overall_errors
        )
    }

    fn generate_scaling_summary_markdown(&self) -> String {
        format!(r#"**Scaling Test Summary:** This test evaluated system performance across {} concurrency levels, from {} to {} concurrent connections. Total of {} requests were processed across all scenarios.

**Performance Characteristics:**
- **Linear Scaling:** {} (Performance improved consistently with load)
- **Peak Performance:** Achieved at {} concurrent connections  
- **Degradation Point:** Performance issues start at {} concurrent connections
- **Resource Efficiency:** {:.1}x improvement from baseline to peak"#,
            self.scenarios.len(),
            self.scenarios.first().map(|s| s.concurrency).unwrap_or(0),
            self.scenarios.last().map(|s| s.concurrency).unwrap_or(0),
            self.overall_requests,
            if self.is_linear_scaling() { "Yes" } else { "No" },
            self.get_best_performing_concurrency(),
            self.get_breaking_point(),
            self.get_scalability_factor()
        )
    }

    fn generate_scaling_table_markdown(&self) -> String {
        let mut table = String::new();
        
        for scenario in &self.scenarios {
            let performance_score = self.calculate_performance_score(scenario);
            table.push_str(&format!(
                "| {} | {} | {:.2} | {:.2} | {:.2} | {} | {:.2} | {:.1} |\n",
                scenario.concurrency,
                scenario.total_requests,
                scenario.success_rate,
                scenario.rps,
                scenario.mean_latency,
                scenario.p95_latency,
                scenario.duration_seconds,
                performance_score
            ));
        }
        
        table
    }

    fn get_bottleneck_analysis(&self) -> String {
        if self.scenarios.len() < 2 {
            return "Insufficient data for bottleneck analysis".to_string();
        }
        
        let breaking_point = self.get_breaking_point();
        let best_performing = self.get_best_performing_concurrency();
        
        if breaking_point <= best_performing {
            "System maintains stable performance across all tested concurrency levels".to_string()
        } else {
            format!("Performance degradation observed beyond {} concurrent connections. Consider investigating resource constraints (CPU, memory, database connections, or network bandwidth)", best_performing)
        }
    }

    fn generate_scaling_summary(&self) -> String {
        format!(r#"
        <p><strong>Scaling Test Summary:</strong> This test evaluated system performance across {} concurrency levels, 
        from {} to {} concurrent connections. Total of {} requests were processed across all scenarios.</p>
        
        <h4>Performance Characteristics:</h4>
        <ul>
            <li><strong>Linear Scaling:</strong> {} (Performance improved consistently with load)</li>
            <li><strong>Peak Performance:</strong> Achieved at {} concurrent connections</li>
            <li><strong>Degradation Point:</strong> Performance issues start at {} concurrent connections</li>
            <li><strong>Resource Efficiency:</strong> {:.1}x improvement from baseline to peak</li>
        </ul>"#,
            self.scenarios.len(),
            self.scenarios.first().map(|s| s.concurrency).unwrap_or(0),
            self.scenarios.last().map(|s| s.concurrency).unwrap_or(0),
            self.overall_requests,
            if self.is_linear_scaling() { "Yes" } else { "No" },
            self.get_best_performing_concurrency(),
            self.get_breaking_point(),
            self.get_scalability_factor()
        )
    }

    fn get_best_performing_concurrency(&self) -> usize {
        self.scenarios.iter()
            .max_by(|a, b| {
                let score_a = self.calculate_performance_score(a);
                let score_b = self.calculate_performance_score(b);
                score_a.partial_cmp(&score_b).unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|s| s.concurrency)
            .unwrap_or(0)
    }

    fn get_breaking_point(&self) -> usize {
        // Find where success rate drops below 95% or latency increases significantly
        for (i, scenario) in self.scenarios.iter().enumerate() {
            if scenario.success_rate < 95.0 || 
               (i > 0 && scenario.mean_latency > self.scenarios[i-1].mean_latency * 2.0) {
                return scenario.concurrency;
            }
        }
        self.scenarios.last().map(|s| s.concurrency).unwrap_or(0)
    }

    fn get_optimal_range(&self) -> (usize, usize) {
        let best = self.get_best_performing_concurrency();
        let breaking = self.get_breaking_point();
        
        // Optimal range is typically from 50% of best to breaking point
        let start = (best as f64 * 0.5) as usize;
        let end = if breaking > best { breaking } else { best };
        
        (start.max(self.scenarios.first().map(|s| s.concurrency).unwrap_or(0)), end)
    }

    fn get_scalability_factor(&self) -> f64 {
        if self.scenarios.len() < 2 {
            return 1.0;
        }
        
        let first = &self.scenarios[0];
        let best = self.scenarios.iter()
            .max_by(|a, b| a.rps.partial_cmp(&b.rps).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(first);
            
        if first.rps > 0.0 {
            best.rps / first.rps
        } else {
            1.0
        }
    }

    fn is_linear_scaling(&self) -> bool {
        if self.scenarios.len() < 3 {
            return true;
        }
        
        // Check if RPS generally increases with concurrency
        let mut increasing_count = 0;
        for i in 1..self.scenarios.len() {
            if self.scenarios[i].rps >= self.scenarios[i-1].rps * 0.9 { // Allow 10% tolerance
                increasing_count += 1;
            }
        }
        
        increasing_count as f64 / (self.scenarios.len() - 1) as f64 >= 0.7 // 70% should be increasing
    }

    fn calculate_performance_score(&self, scenario: &crate::models::ScenarioResult) -> f64 {
        // Performance score based on RPS, success rate, and latency
        let rps_score = scenario.rps / 100.0; // Normalize RPS
        let success_penalty = (100.0 - scenario.success_rate) / 10.0; // Penalty for failures
        let latency_penalty = scenario.mean_latency / 100.0; // Penalty for high latency
        
        (rps_score - success_penalty - latency_penalty).max(0.0)
    }

    fn get_recommendations(&self) -> String {
        let (optimal_start, optimal_end) = self.get_optimal_range();
        let breaking_point = self.get_breaking_point();
        let scalability_factor = self.get_scalability_factor();
        
        format!(r#"Based on the scaling analysis:

1. **Optimal Operating Range:** {} - {} concurrent connections provide the best balance of throughput and latency
2. **Performance Degradation:** Monitor closely beyond {} concurrent connections
3. **Resource Planning:** System shows {:.1}x scaling capability from baseline to peak performance
4. **Bottleneck Analysis:** {}"#,
            optimal_start, optimal_end, breaking_point, scalability_factor, self.get_bottleneck_analysis())
    }

    fn generate_scaling_comparison_table(&self) -> String {
        let mut rows = String::new();
        
        for scenario in &self.scenarios {
            let performance_score = self.calculate_performance_score(scenario);
            rows.push_str(&format!(r#"
            <tr>
                <td>{}</td>
                <td>{}</td>
                <td>{:.2}</td>
                <td>{:.2}</td>
                <td>{:.2}</td>
                <td>{}</td>
                <td>{}</td>
                <td>{:.2}</td>
                <td>{:.1}</td>
            </tr>"#,
                scenario.concurrency,
                scenario.total_requests,
                scenario.success_rate,
                scenario.rps,
                scenario.mean_latency,
                scenario.p95_latency,
                scenario.p99_latency,
                scenario.duration_seconds,
                performance_score
            ));
        }
        
        rows
    }
}

pub fn save_report(report: &LoadTestReport) -> Result<(), Box<dyn std::error::Error>> {
    // Save HTML report as index.html
    let mut html_file = File::create("index.html")?;
    html_file.write_all(report.generate_html().as_bytes())?;
    
    // Save Markdown report as README.md
    let mut md_file = File::create("README.md")?;
    md_file.write_all(report.generate_markdown().as_bytes())?;
    
    // Save JSON data for further analysis
    let mut json_file = File::create("load_test_data.json")?;
    json_file.write_all(serde_json::to_string_pretty(report)?.as_bytes())?;
    
    println!("\n📄 Reports generated:");
    println!("   - index.html");
    println!("   - README.md");
    println!("   - load_test_data.json");
    
    Ok(())
}
