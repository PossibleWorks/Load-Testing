# Load Test Report

*Generated on: 2025-07-14 16:07:48*  
*Test Duration: 7.47 seconds*

## 1. Test Objectives

This load test was conducted to evaluate the performance and scalability of the API endpoints under various load conditions. The test aimed to:

- Determine the maximum sustainable request rate
- Measure response times under different concurrency levels  
- Identify potential bottlenecks and failure points
- Validate system stability under stress conditions

## 2. Test Configuration

| Parameter | Value |
|-----------|-------|
| Base URL | https://bev3-dev.lykkebook.com/api |
| Total Endpoints Tested | 18 |
| Test Start Time | 2025-07-14 16:07:40 |
| Test End Time | 2025-07-14 16:07:48 |

## 3. Overall Results Summary

### Key Metrics
- **Total Requests:** 900
- **Success Rate:** 100.00%
- **Average RPS:** 120.48
- **Mean Latency:** 3880.67ms

### Detailed Results

| Metric | Value |
|--------|-------|
| Total Requests | 900 |
| Successful Requests | 900 |
| Failed Requests | 0 |
| Success Rate | 100.00% |
| Average RPS | 120.48 req/sec |
| Mean Latency | 3880.67ms |
| 95th Percentile Latency | 6057ms |
| 99th Percentile Latency | 6057ms |

## 4. Scenario Results


### Scenario 1 - Concurrency: 50

| Metric | Value |
|--------|-------|
| Concurrency | 50 |
| Total Requests | 900 |
| Success Rate | 100.00% |
| RPS | 120.49 |
| Mean Latency | 3881.08ms |
| P95 Latency | 6472ms |
| Duration | 7.47s |

#### Endpoint Details:

| Endpoint | Requests | Success | Errors | Success Rate | Mean Latency | P95 Latency |
|----------|----------|---------|--------|--------------|--------------|-------------|
| /auth/get-tenants | 50 | 50 | 0 | 100.00% | 4068.08ms | 6663ms |
| /user/me | 50 | 50 | 0 | 100.00% | 2029.32ms | 2882ms |
| /user/active-and-inactive-users | 50 | 50 | 0 | 100.00% | 3103.62ms | 5698ms |
| /user/get-user-skills/0f75671e-a024-43bc-82aa-1f454f39ffd5 | 50 | 50 | 0 | 100.00% | 3017.10ms | 5368ms |
| /user/team/0f75671e-a024-43bc-82aa-1f454f39ffd5 | 50 | 50 | 0 | 100.00% | 4575.54ms | 5901ms |
| /user/chat-users/ | 50 | 50 | 0 | 100.00% | 4680.44ms | 6618ms |
| /user/all-users | 50 | 50 | 0 | 100.00% | 2824.40ms | 6010ms |
| /user/user-progress-details/0f75671e-a024-43bc-82aa-1f454f39ffd5/5b39887b-6659-4d87-975f-508917131ea3/a915cb6e-0974-4fb8-9553-81f04ce7ca45 | 50 | 50 | 0 | 100.00% | 6057.48ms | 7331ms |
| /goals/user-goals/5b39887b-6659-4d87-975f-508917131ea3/0f75671e-a024-43bc-82aa-1f454f39ffd5 | 50 | 50 | 0 | 100.00% | 3528.68ms | 5528ms |
| /goals/manager-reportee/5b39887b-6659-4d87-975f-508917131ea3/0f75671e-a024-43bc-82aa-1f454f39ffd5 | 50 | 50 | 0 | 100.00% | 3410.14ms | 6123ms |
| /goals/eligible-reportees-for-cascade/5b39887b-6659-4d87-975f-508917131ea3 | 50 | 50 | 0 | 100.00% | 4904.58ms | 6293ms |
| /goals/get-goal-categories | 50 | 50 | 0 | 100.00% | 4221.14ms | 6397ms |
| /goals/get-objectives-akrs-and-quadrants/a915cb6e-0974-4fb8-9553-81f04ce7ca45 | 50 | 50 | 0 | 100.00% | 3209.02ms | 6745ms |
| /goals/detailed-goals/5b39887b-6659-4d87-975f-508917131ea3/0f75671e-a024-43bc-82aa-1f454f39ffd5 | 50 | 50 | 0 | 100.00% | 3654.58ms | 5932ms |
| /goals/cascade-json/0f75671e-a024-43bc-82aa-1f454f39ffd5/5b39887b-6659-4d87-975f-508917131ea3 | 50 | 50 | 0 | 100.00% | 4277.94ms | 6595ms |
| /periods/previous-current-next | 50 | 50 | 0 | 100.00% | 5412.54ms | 6619ms |
| /periods/5b39887b-6659-4d87-975f-508917131ea3/subperiods | 50 | 50 | 0 | 100.00% | 3878.76ms | 5879ms |
| /cycles/ | 50 | 50 | 0 | 100.00% | 3006.04ms | 5686ms |


## 5. Performance Analysis

### Response Time Analysis
- **Overall success rate:** 100.00% - Excellent
- **Average response time:** 3880.67ms - Marginal  
- **Maximum throughput achieved:** 120.48 requests per second

### Recommendations

1. Consider optimizing slow endpoints and investigating error causes
2. Monitor database connection pool settings if response times degrade with higher concurrency
3. Consider implementing caching strategies for frequently accessed endpoints
4. Set up monitoring and alerting for response times exceeding acceptable thresholds

## 6. Conclusions

Performance improvements are recommended. Focus on optimizing high-latency endpoints and reducing error rates.
