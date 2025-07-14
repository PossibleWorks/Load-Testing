use load_test_rs::{LoadTestConfig, LoadTester, save_report};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Create configuration based on command line arguments
    let config = match args.get(1).map(|s| s.as_str()) {
        Some("quick") => {
            println!("🚀 Running Quick Scaling Test (4 scenarios: 50 -> 100 -> 200 -> 500)");
            LoadTestConfig::new_quick_scaling()
        },
        Some("custom") => {
            let max_concurrency = args.get(2)
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(1000);
            println!("🚀 Running Custom Scaling Test (up to {} concurrent connections)", max_concurrency);
            LoadTestConfig::new_custom_scaling(max_concurrency)
        },
        Some("full") => {
            println!("🚀 Running Full Scaling Test (7 scenarios: 200 -> 400 -> 800 -> 1600 -> 3200 -> 6400 -> 10000)");
            LoadTestConfig::new()
        },
        _ => {
            println!("🚀 Running Full Scaling Test (7 scenarios: 200 -> 400 -> 800 -> 1600 -> 3200 -> 6400 -> 10000)");
            println!("💡 Usage options:");
            println!("  cargo run quick           # Quick test (4 scenarios)");
            println!("  cargo run custom 5000     # Custom max concurrency");
            println!("  cargo run full            # Full scaling test (default)");
            println!();
            LoadTestConfig::new()
        }
    };
    
    // Display test configuration
    println!("📊 Test Configuration:");
    println!("   Base URL: {}", config.base_url);
    println!("   {}", config.get_scaling_description());
    println!("   Total Requests: {}", config.get_total_requests());
    println!("   Estimated Duration: {:.1} minutes", config.estimate_duration_minutes());
    
    println!("\n🎯 Scaling Scenarios:");
    for (i, scenario) in config.scenarios.iter().enumerate() {
        println!("   {}. Concurrency: {}, Requests: {}", 
                 i + 1, scenario.concurrency, scenario.requests);
    }
    println!();
    
    // Create and run load tester
    let load_tester = LoadTester::new(config);
    let report = load_tester.run();

    // Save the reports
    match save_report(&report) {
        Ok(_) => {
            println!("✅ Reports generated successfully!");
            println!("📄 HTML Report: index.html");
            println!("📄 Markdown Report: README.md");
            println!("📄 JSON Data: load_test_data.json");
        },
        Err(e) => {
            eprintln!("❌ Error generating reports: {}", e);
        }
    }
}
