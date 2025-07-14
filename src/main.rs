use load_test_rs::{LoadTestConfig, LoadTester, save_report};

fn main() {
    // Create configuration
    let config = LoadTestConfig::new();
    
    // Create and run load tester
    let load_tester = LoadTester::new(config);
    let report = load_tester.run();

    // Save the reports
    if let Err(e) = save_report(&report) {
        eprintln!("Error generating reports: {}", e);
    }
}
