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
            scenarios: vec![Scenario {
                concurrency: 50,
                requests: 50,
            }],
        }
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
}

impl Default for LoadTestConfig {
    fn default() -> Self {
        Self::new()
    }
}
