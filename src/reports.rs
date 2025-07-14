use crate::models::LoadTestReport;
use std::fs::File;
use std::io::Write;

impl LoadTestReport {
    pub fn generate_html(&self) -> String {
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
        .chart-placeholder {{ border: 1px solid #ddd; height: 300px; margin: 20px 0; padding: 20px; text-align: center; background-color: #f9f9f9; }}
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
        <div class="chart-placeholder">
            <h3>Response Time vs Concurrency</h3>
            <p>Chart showing how response times change with increasing concurrency levels</p>
            <p><em>Note: Interactive charts can be generated using Chart.js or similar libraries</em></p>
        </div>
        <div class="chart-placeholder">
            <h3>Requests Per Second Over Time</h3>
            <p>Chart showing request throughput over the duration of the test</p>
        </div>
    </div>

    <div class="section">
        <h2>6. Conclusions and Recommendations</h2>
        <h3>Performance Analysis:</h3>
        <ul>
            <li>Overall success rate: {:.2}% - {}.</li>
            <li>Average response time: {:.2}ms - {}.</li>
            <li>Maximum throughput achieved: {:.2} requests per second.</li>
        </ul>
        
        <h3>Recommendations:</h3>
        <ul>
            <li>{}</li>
            <li>Monitor database connection pool settings if response times degrade with higher concurrency</li>
            <li>Consider implementing caching strategies for frequently accessed endpoints</li>
            <li>Set up monitoring and alerting for response times exceeding acceptable thresholds</li>
        </ul>
    </div>
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
            self.overall_success_rate,
            if self.overall_success_rate >= 99.0 { "Excellent" } else if self.overall_success_rate >= 95.0 { "Good" } else { "Needs improvement" },
            self.overall_mean_latency,
            if self.overall_mean_latency <= 1000.0 { "Acceptable" } else if self.overall_mean_latency <= 5000.0 { "Marginal" } else { "Poor" },
            self.overall_rps,
            if self.overall_success_rate >= 99.0 && self.overall_mean_latency <= 1000.0 { 
                "System performance is within acceptable parameters for current load levels" 
            } else { 
                "Consider optimizing slow endpoints and investigating error causes" 
            }
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

## 5. Performance Analysis

### Response Time Analysis
- **Overall success rate:** {:.2}% - {}
- **Average response time:** {:.2}ms - {}  
- **Maximum throughput achieved:** {:.2} requests per second

### Recommendations

1. {}
2. Monitor database connection pool settings if response times degrade with higher concurrency
3. Consider implementing caching strategies for frequently accessed endpoints
4. Set up monitoring and alerting for response times exceeding acceptable thresholds

## 6. Conclusions

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
            self.generate_scenario_markdown(),
            self.overall_success_rate,
            if self.overall_success_rate >= 99.0 { "Excellent" } else if self.overall_success_rate >= 95.0 { "Good" } else { "Needs improvement" },
            self.overall_mean_latency,
            if self.overall_mean_latency <= 1000.0 { "Acceptable" } else if self.overall_mean_latency <= 5000.0 { "Marginal" } else { "Poor" },
            self.overall_rps,
            if self.overall_success_rate >= 99.0 && self.overall_mean_latency <= 1000.0 { 
                "System performance is within acceptable parameters for current load levels" 
            } else { 
                "Consider optimizing slow endpoints and investigating error causes" 
            },
            if self.overall_success_rate >= 99.0 && self.overall_mean_latency <= 1000.0 {
                "The system demonstrates good performance characteristics under the tested load conditions. Continue monitoring in production environments."
            } else {
                "Performance improvements are recommended. Focus on optimizing high-latency endpoints and reducing error rates."
            }
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
{}
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
