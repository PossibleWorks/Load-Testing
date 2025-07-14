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
        <h2>4. Scenario Results</h2>
        {}
    </div>

    <div class="section">
        <h2>5. Performance Charts</h2>
        
        <div class="chart-container">
            <h3>Response Time vs Concurrency</h3>
            <canvas id="latencyChart" width="800" height="400"></canvas>
        </div>
        
        <div class="chart-container">
            <h3>Requests Per Second by Scenario</h3>
            <canvas id="rpsChart" width="800" height="400"></canvas>
        </div>
        
        <div class="chart-container">
            <h3>Success Rate Overview</h3>
            <canvas id="successChart" width="800" height="400"></canvas>
        </div>
        
        <div class="chart-container">
            <h3>Latency Distribution by Scenario</h3>
            <canvas id="percentileChart" width="800" height="400"></canvas>
        </div>
        
        <div class="chart-container">
            <h3>Endpoint Performance Comparison</h3>
            <canvas id="endpointChart" width="800" height="600"></canvas>
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
        format!(r#"# Load Test Report

*Generated on: {}*  
*Test Duration: {:.2} seconds*

## 1. Test Objectives

This load test was conducted to evaluate the performance and scalability of the API endpoints under various load conditions. The test aimed to:

- Determine the maximum sustainable request rate
- Measure response times under different concurrency levels  
- Identify potential bottlenecks and failure points
- Validate system stability under stress conditions

## 2. Test Configuration

| Parameter | Value |
|-----------|-------|
| Base URL | {} |
| Total Endpoints Tested | {} |
| Test Start Time | {} |
| Test End Time | {} |

## 3. Overall Results Summary

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

## 4. Scenario Results

{}
"#,
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
            self.generate_scenario_markdown()
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
        // Extract data for charts
        let scenario_labels: Vec<String> = self.scenarios.iter()
            .map(|s| format!("\"Concurrency {}\"", s.concurrency))
            .collect();
        
        let mean_latencies: Vec<String> = self.scenarios.iter()
            .map(|s| s.mean_latency.to_string())
            .collect();
            
        let p95_latencies: Vec<String> = self.scenarios.iter()
            .map(|s| s.p95_latency.to_string())
            .collect();
            
        let p99_latencies: Vec<String> = self.scenarios.iter()
            .map(|s| s.p99_latency.to_string())
            .collect();
            
        let rps_values: Vec<String> = self.scenarios.iter()
            .map(|s| s.rps.to_string())
            .collect();
            
        // Get endpoint data from the first scenario (or aggregate if needed)
        let endpoint_data = if let Some(first_scenario) = self.scenarios.first() {
            first_scenario.endpoints.iter()
                .map(|ep| (
                    format!("\"{}\"", ep.endpoint.replace('"', "\\\"")[..ep.endpoint.len().min(30)].to_string()),
                    ep.mean_latency.to_string(),
                    ep.success_rate.to_string()
                ))
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };
        
        let endpoint_labels: Vec<String> = endpoint_data.iter().map(|(label, _, _)| label.clone()).collect();
        let endpoint_latencies: Vec<String> = endpoint_data.iter().map(|(_, latency, _)| latency.clone()).collect();
        let endpoint_success_rates: Vec<String> = endpoint_data.iter().map(|(_, _, success)| success.clone()).collect();

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

        // Response Time vs Concurrency Chart
        const latencyCtx = document.getElementById('latencyChart').getContext('2d');
        new Chart(latencyCtx, {{
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
                }}, {{
                    label: 'P99 Latency (ms)',
                    data: [{}],
                    borderColor: chartColors.danger,
                    backgroundColor: chartColors.danger + '20',
                    tension: 0.1,
                    fill: false
                }}]
            }},
            options: {{
                responsive: true,
                plugins: {{
                    title: {{
                        display: true,
                        text: 'Response Time Performance by Concurrency Level'
                    }},
                    legend: {{
                        display: true
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
                            text: 'Test Scenarios'
                        }}
                    }}
                }}
            }}
        }});

        // RPS Chart
        const rpsCtx = document.getElementById('rpsChart').getContext('2d');
        new Chart(rpsCtx, {{
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
                        text: 'Throughput by Test Scenario'
                    }}
                }},
                scales: {{
                    y: {{
                        beginAtZero: true,
                        title: {{
                            display: true,
                            text: 'Requests Per Second'
                        }}
                    }}
                }}
            }}
        }});

        // Success Rate Chart
        const successCtx = document.getElementById('successChart').getContext('2d');
        new Chart(successCtx, {{
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
                        text: 'Overall Request Success Rate'
                    }},
                    legend: {{
                        position: 'bottom'
                    }}
                }}
            }}
        }});

        // Percentile Chart (Radar)
        const percentileCtx = document.getElementById('percentileChart').getContext('2d');
        new Chart(percentileCtx, {{
            type: 'radar',
            data: {{
                labels: [{}],
                datasets: [{{
                    label: 'Mean Latency (ms)',
                    data: [{}],
                    borderColor: chartColors.primary,
                    backgroundColor: chartColors.primary + '20',
                    pointBackgroundColor: chartColors.primary
                }}, {{
                    label: 'P95 Latency (ms)',
                    data: [{}],
                    borderColor: chartColors.warning,
                    backgroundColor: chartColors.warning + '20',
                    pointBackgroundColor: chartColors.warning
                }}]
            }},
            options: {{
                responsive: true,
                plugins: {{
                    title: {{
                        display: true,
                        text: 'Latency Distribution Across Scenarios'
                    }}
                }},
                scales: {{
                    r: {{
                        beginAtZero: true,
                        title: {{
                            display: true,
                            text: 'Latency (ms)'
                        }}
                    }}
                }}
            }}
        }});

        // Endpoint Performance Chart
        const endpointCtx = document.getElementById('endpointChart').getContext('2d');
        new Chart(endpointCtx, {{
            type: 'bar',
            data: {{
                labels: [{}],
                datasets: [{{
                    label: 'Mean Latency (ms)',
                    data: [{}],
                    backgroundColor: chartColors.primary,
                    yAxisID: 'y'
                }}, {{
                    label: 'Success Rate (%)',
                    data: [{}],
                    backgroundColor: chartColors.success,
                    yAxisID: 'y1',
                    type: 'line',
                    borderColor: chartColors.success,
                    tension: 0.1
                }}]
            }},
            options: {{
                responsive: true,
                interaction: {{
                    mode: 'index',
                    intersect: false,
                }},
                plugins: {{
                    title: {{
                        display: true,
                        text: 'Endpoint Performance: Latency vs Success Rate'
                    }}
                }},
                scales: {{
                    x: {{
                        title: {{
                            display: true,
                            text: 'API Endpoints'
                        }},
                        ticks: {{
                            maxRotation: 45,
                            minRotation: 45
                        }}
                    }},
                    y: {{
                        type: 'linear',
                        display: true,
                        position: 'left',
                        title: {{
                            display: true,
                            text: 'Mean Latency (ms)'
                        }}
                    }},
                    y1: {{
                        type: 'linear',
                        display: true,
                        position: 'right',
                        title: {{
                            display: true,
                            text: 'Success Rate (%)'
                        }},
                        grid: {{
                            drawOnChartArea: false,
                        }},
                        min: 0,
                        max: 100
                    }}
                }}
            }}
        }});
        "#,
            scenario_labels.join(", "),
            mean_latencies.join(", "),
            p95_latencies.join(", "),
            p99_latencies.join(", "),
            scenario_labels.join(", "),
            rps_values.join(", "),
            self.overall_requests - self.overall_errors,
            self.overall_errors,
            scenario_labels.join(", "),
            mean_latencies.join(", "),
            p95_latencies.join(", "),
            endpoint_labels.join(", "),
            endpoint_latencies.join(", "),
            endpoint_success_rates.join(", ")
        )
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
