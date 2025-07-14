use crate::models::Scenario;

pub struct LoadTestConfig {
    pub base_url: String,
    pub auth_header: String,
    pub tenant_header: String,
    pub user_id: String,
    pub period_id: String,
    pub cycle_id: String,
    pub scenarios: Vec<Scenario>,
}

impl LoadTestConfig {
    pub fn new() -> Self {
        Self {
            base_url: "https://bev3-dev.lykkebook.com/api".to_string(),
            auth_header: "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3N1ZXIiOiJwb3NzaWJsZXdvcmtzLmNvbSIsImF1ZGllbmNlIjoicG9zc2libGV3b3Jrcy5jb20iLCJmaXJzdE5hbWUiOiJjZW8iLCJsYXN0TmFtZSI6ImNsaWVudF9xYSIsImVtYWlsIjoiY2VvQHB3LmNvbSIsImlkIjoiODQ2OTZjYzktMzViNy00NGYzLWI4YTctMWZhZDI2NmMyYTkwIiwidGVuYW50VXNlcklkIjoiMGY3NTY3MWUtYTAyNC00M2JjLTgyYWEtMWY0NTRmMzlmZmQ1Iiwic3ViIjoiMGY3NTY3MWUtYTAyNC00M2JjLTgyYWEtMWY0NTRmMzlmZmQ1IiwiaWF0IjoxNzUyMzc4ODMxMDI0LCJhdXRoX3RpbWUiOjE3NTIzNzg4MzEwMjQsInJvbGUiOiIyIiwidGVuYW50aWQiOiJhYzAwNzBhMS1jMjdlLTQ4ZWYtYWFmMi00MzAzYmRmY2UyYzUiLCJzdGF0dXMiOiJhY3RpdmUiLCJleHAiOjE3NTIzNzg5MTc0MjR9.hRhFxHtVcKQZZVDKb4VpXVZUj2wnnWQc0Dn0eFc9NWQ".to_string(),
            tenant_header: "ac0070a1-c27e-48ef-aaf2-4303bdfce2c5".to_string(),
            user_id: "0f75671e-a024-43bc-82aa-1f454f39ffd5".to_string(),
            period_id: "5b39887b-6659-4d87-975f-508917131ea3".to_string(),
            cycle_id: "a915cb6e-0974-4fb8-9553-81f04ce7ca45".to_string(),
            scenarios: Self::generate_scaling_scenarios(),
        }
    }

    /// Generate scaling scenarios from 200 to 10,000 connections
    /// Pattern: 200 -> 400 -> 800 -> 1600 -> 3200 -> 6400 -> 10000
    fn generate_scaling_scenarios() -> Vec<Scenario> {
        let mut scenarios = Vec::new();
        
        // Start with smaller increments for fine-grained testing
        let base_levels = vec![200, 400, 800];
        
        // Add exponential scaling: 1600, 3200, 6400
        let mut current = 1600;
        let mut exponential_levels = Vec::new();
        while current <= 6400 {
            exponential_levels.push(current);
            current *= 2;
        }
        
        // Add final target
        let final_levels = vec![10000];
        
        // Combine all levels
        let all_levels = [base_levels, exponential_levels, final_levels].concat();
        
        for concurrency in all_levels {
            scenarios.push(Scenario {
                concurrency,
                requests: concurrency, // Match requests to concurrency for consistent load
            });
        }
        
        scenarios
    }

    /// Generate a simple scaling scenario for quick testing (fewer levels)
    pub fn new_quick_scaling() -> Self {
        let mut config = Self::new();
        config.scenarios = vec![
            Scenario { concurrency: 50, requests: 50 },
            Scenario { concurrency: 100, requests: 100 },
            Scenario { concurrency: 200, requests: 200 },
            Scenario { concurrency: 500, requests: 500 },
        ];
        config
    }

    /// Generate a custom scaling scenario with specified max concurrency
    pub fn new_custom_scaling(max_concurrency: usize) -> Self {
        let mut config = Self::new();
        config.scenarios = Self::generate_custom_scaling_scenarios(max_concurrency);
        config
    }

    fn generate_custom_scaling_scenarios(max_concurrency: usize) -> Vec<Scenario> {
        let mut scenarios = Vec::new();
        let mut current = 200;
        
        // Generate scenarios doubling each time until we reach max
        while current <= max_concurrency {
            scenarios.push(Scenario {
                concurrency: current,
                requests: current,
            });
            
            if current >= max_concurrency {
                break;
            }
            
            current = if current < 1000 { current * 2 } else { current + 1000 };
            if current > max_concurrency {
                current = max_concurrency;
            }
        }
        
        scenarios
    }

    pub fn get_endpoints(&self) -> Vec<String> {
        vec![
            "/auth/get-tenants".to_string(),
            "/user/me".to_string(),
            "/user/active-and-inactive-users".to_string(),
            format!("/user/get-user-skills/{}", self.user_id),
            format!("/user/team/{}", self.user_id),
            "/user/chat-users/".to_string(),
            "/user/all-users".to_string(),
            format!(
                "/user/user-progress-details/{}/{}/{}",
                self.user_id, self.period_id, self.cycle_id
            ),
            format!("/goals/user-goals/{}/{}", self.period_id, self.user_id),
            format!("/goals/manager-reportee/{}/{}", self.period_id, self.user_id),
            format!("/goals/eligible-reportees-for-cascade/{}", self.period_id),
            "/goals/get-goal-categories".to_string(),
            format!("/goals/get-objectives-akrs-and-quadrants/{}", self.cycle_id),
            format!("/goals/detailed-goals/{}/{}", self.period_id, self.user_id),
            format!("/goals/cascade-json/{}/{}", self.user_id, self.period_id),
            "/periods/previous-current-next".to_string(),
            format!("/periods/{}/subperiods", self.period_id),
            "/cycles/".to_string(),
        ]
    }

    /// Get a description of the current scaling configuration
    pub fn get_scaling_description(&self) -> String {
        let concurrency_levels: Vec<usize> = self.scenarios.iter()
            .map(|s| s.concurrency)
            .collect();
        
        format!("Scaling from {} to {} concurrent connections ({} levels)",
                concurrency_levels.first().unwrap_or(&0),
                concurrency_levels.last().unwrap_or(&0),
                concurrency_levels.len())
    }

    /// Get total estimated requests across all scenarios
    pub fn get_total_requests(&self) -> usize {
        self.scenarios.iter().map(|s| s.requests).sum()
    }

    /// Get estimated test duration (rough estimate based on request count)
    pub fn estimate_duration_minutes(&self) -> f64 {
        let total_requests = self.get_total_requests();
        // Rough estimate: assume 10 requests per second average
        let estimated_seconds = total_requests as f64 / 10.0;
        estimated_seconds / 60.0
    }
}

impl Default for LoadTestConfig {
    fn default() -> Self {
        Self::new()
    }
}
