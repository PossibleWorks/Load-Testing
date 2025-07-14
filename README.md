# Load Test Report

*Generated on: 2025-07-14 16:29:23*  
*Test Duration: 12.12 seconds*

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
| Test Start Time | 2025-07-14 16:29:11 |
| Test End Time | 2025-07-14 16:29:23 |

## 3. Overall Results Summary

### Key Metrics
- **Total Requests:** 900
- **Success Rate:** 100.00%
- **Average RPS:** 74.23
- **Mean Latency:** 5440.17ms

### Detailed Results

| Metric | Value |
|--------|-------|
| Total Requests | 900 |
| Successful Requests | 900 |
| Failed Requests | 0 |
| Success Rate | 100.00% |
| Average RPS | 74.23 req/sec |
| Mean Latency | 5440.17ms |
| 95th Percentile Latency | 10020ms |
| 99th Percentile Latency | 10020ms |

## 4. Scenario Results


### Scenario 1 - Concurrency: 50

| Metric | Value |
|--------|-------|
| Concurrency | 50 |
| Total Requests | 900 |
| Success Rate | 100.00% |
| RPS | 74.24 |
| Mean Latency | 5440.64ms |
| P95 Latency | 10443ms |
| Duration | 12.12s |

#### Endpoint Details:

| Endpoint | Requests | Success | Errors | Success Rate | Mean Latency | P95 Latency |
|----------|----------|---------|--------|--------------|--------------|-------------|
| | /auth/get-tenants | 50 | 50 | 0 | 100.00% | 6761.38ms | 12106ms |
| /user/me | 50 | 50 | 0 | 100.00% | 4050.72ms | 7604ms |
| /user/active-and-inactive-users | 50 | 50 | 0 | 100.00% | 4633.66ms | 10454ms |
| /user/get-user-skills/0f75671e-a024-43bc-82aa-1f454f39ffd5 | 50 | 50 | 0 | 100.00% | 3802.26ms | 7711ms |
| /user/team/0f75671e-a024-43bc-82aa-1f454f39ffd5 | 50 | 50 | 0 | 100.00% | 4122.52ms | 10087ms |
| /user/chat-users/ | 50 | 50 | 0 | 100.00% | 4582.94ms | 9020ms |
| /user/all-users | 50 | 50 | 0 | 100.00% | 5767.26ms | 10223ms |
| /user/user-progress-details/0f75671e-a024-43bc-82aa-1f454f39ffd5/5b39887b-6659-4d87-975f-508917131ea3/a915cb6e-0974-4fb8-9553-81f04ce7ca45 | 50 | 50 | 0 | 100.00% | 10020.96ms | 11809ms |
| /goals/user-goals/5b39887b-6659-4d87-975f-508917131ea3/0f75671e-a024-43bc-82aa-1f454f39ffd5 | 50 | 50 | 0 | 100.00% | 4047.52ms | 8525ms |
| /goals/manager-reportee/5b39887b-6659-4d87-975f-508917131ea3/0f75671e-a024-43bc-82aa-1f454f39ffd5 | 50 | 50 | 0 | 100.00% | 4296.40ms | 7714ms |
| /goals/eligible-reportees-for-cascade/5b39887b-6659-4d87-975f-508917131ea3 | 50 | 50 | 0 | 100.00% | 4203.08ms | 8540ms |
| /goals/get-goal-categories | 50 | 50 | 0 | 100.00% | 4889.60ms | 8674ms |
| /goals/get-objectives-akrs-and-quadrants/a915cb6e-0974-4fb8-9553-81f04ce7ca45 | 50 | 50 | 0 | 100.00% | 5608.54ms | 9022ms |
| /goals/detailed-goals/5b39887b-6659-4d87-975f-508917131ea3/0f75671e-a024-43bc-82aa-1f454f39ffd5 | 50 | 50 | 0 | 100.00% | 6555.78ms | 10029ms |
| /goals/cascade-json/0f75671e-a024-43bc-82aa-1f454f39ffd5/5b39887b-6659-4d87-975f-508917131ea3 | 50 | 50 | 0 | 100.00% | 6149.62ms | 9653ms |
| /periods/previous-current-next | 50 | 50 | 0 | 100.00% | 7672.14ms | 10443ms |
| /periods/5b39887b-6659-4d87-975f-508917131ea3/subperiods | 50 | 50 | 0 | 100.00% | 4473.10ms | 8698ms |
| /cycles/ | 50 | 50 | 0 | 100.00% | 6294.00ms | 9250ms | |

