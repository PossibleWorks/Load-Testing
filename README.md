# Load Test Report - Scaling Analysis

*Generated on: 2025-07-27 11:49:53*  
*Test Duration: 10394.87 seconds*

## 1. Test Objectives

This load test was conducted to evaluate the performance and scalability of the API endpoints under various load conditions. The test aimed to:

- Determine the maximum sustainable request rate
- Measure response times under different concurrency levels  
- Identify potential bottlenecks and failure points
- Validate system stability under stress conditions
- Analyze scaling characteristics from 200 to 10000 concurrent connections

## 2. Test Configuration

| Parameter | Value |
|-----------|-------|
| Base URL | https://bev3-dev.lykkebook.com/api |
| Total Endpoints Tested | 18 |
| Scaling Levels | 7 scenarios |
| Concurrency Range | 200 - 10000 connections |
| Total Requests | 406800 |
| Test Start Time | 2025-07-27 08:56:38 |
| Test End Time | 2025-07-27 11:49:53 |

## 3. Scaling Analysis & Performance Summary

**Auto-Scaling Performance Test:** This test evaluated system auto-scaling effectiveness across 7 concurrency levels, from 200 to 10000 concurrent connections. Total of 406800 requests were processed across all scenarios.

**Auto-Scaling Characteristics:**
- **Scaling Effectiveness:** Excellent (Performance improved consistently as instances scaled)
- **Peak Performance:** Achieved at 10000 concurrent connections (optimal auto-scaling)
- **Scaling Threshold:** Auto-scaling activated around 10000 concurrent connections
- **Infrastructure Efficiency:** 3.1x throughput improvement through automatic instance provisioning

### Performance Trends

| Concurrency | Total Requests | Success Rate (%) | RPS | Mean Latency (ms) | P95 Latency (ms) | Duration (s) | Performance Score |
|-------------|----------------|------------------|-----|-------------------|------------------|--------------|-------------------|
| 200 | 3600 | 97.22 | 13.76 | 3372.91 | 14415 | 261.54 | 0.0 |
| 400 | 7200 | 99.83 | 25.02 | 1864.69 | 6741 | 287.72 | 0.0 |
| 800 | 14400 | 99.99 | 39.47 | 1204.51 | 6504 | 364.87 | 0.0 |
| 1600 | 28800 | 99.98 | 39.89 | 1223.11 | 6579 | 721.96 | 0.0 |
| 3200 | 57600 | 99.99 | 42.36 | 1163.77 | 6268 | 1359.68 | 0.0 |
| 6400 | 115200 | 99.99 | 39.53 | 1255.36 | 6906 | 2914.51 | 0.0 |
| 10000 | 180000 | 99.99 | 40.15 | 1238.65 | 6894 | 4483.73 | 0.0 |


### Key Performance Insights

- **Best Performing Concurrency:** 10000 connections
- **Breaking Point:** 10000 connections  
- **Optimal Load Range:** 5000 - 10000 connections
- **Scalability Factor:** 3.1x improvement from baseline to peak
- **Linear Scaling:** Yes

## 4. Overall Results Summary

### Key Metrics
- **Total Requests:** 406800
- **Success Rate:** 99.96%
- **Average RPS:** 39.13
- **Mean Latency:** 1259.91ms

### Detailed Results

| Metric | Value |
|--------|-------|
| Total Requests | 406800 |
| Successful Requests | 406635 |
| Failed Requests | 165 |
| Success Rate | 99.96% |
| Average RPS | 39.13 req/sec |
| Mean Latency | 1259.91ms |
| 95th Percentile Latency | 6687ms |
| 99th Percentile Latency | 7407ms |

## 5. Detailed Scenario Results


### Scenario 1 - Concurrency: 200

| Metric | Value |
|--------|-------|
| Concurrency | 200 |
| Total Requests | 3600 |
| Success Rate | 97.22% |
| RPS | 13.76 |
| Mean Latency | 3372.91ms |
| P95 Latency | 14415ms |
| Duration | 261.54s |

#### Endpoint Details:

| Endpoint | Requests | Success | Errors | Success Rate | Mean Latency | P95 Latency |
|----------|----------|---------|--------|--------------|--------------|-------------|
| | /auth/get-tenants | 200 | 200 | 0 | 100.00% | 1953.12ms | 2798ms |
| /user/me | 200 | 100 | 100 | 50.00% | 17463.65ms | 30000ms |
| /user/active-and-inactive-users | 200 | 200 | 0 | 100.00% | 3904.24ms | 4808ms |
| /user/get-user-skills/0f75671e-a024-43bc-82aa-1f454f39ffd5 | 200 | 200 | 0 | 100.00% | 1246.37ms | 1959ms |
| /user/team/0f75671e-a024-43bc-82aa-1f454f39ffd5 | 200 | 200 | 0 | 100.00% | 864.21ms | 1262ms |
| /user/chat-users/ | 200 | 200 | 0 | 100.00% | 3203.01ms | 3674ms |
| /user/all-users | 200 | 200 | 0 | 100.00% | 1443.72ms | 1798ms |
| /user/user-progress-details/0f75671e-a024-43bc-82aa-1f454f39ffd5/5b39887b-6659-4d87-975f-508917131ea3/a915cb6e-0974-4fb8-9553-81f04ce7ca45 | 200 | 200 | 0 | 100.00% | 14346.48ms | 14892ms |
| /goals/user-goals/5b39887b-6659-4d87-975f-508917131ea3/0f75671e-a024-43bc-82aa-1f454f39ffd5 | 200 | 200 | 0 | 100.00% | 1712.19ms | 2067ms |
| /goals/manager-reportee/5b39887b-6659-4d87-975f-508917131ea3/0f75671e-a024-43bc-82aa-1f454f39ffd5 | 200 | 200 | 0 | 100.00% | 2894.64ms | 3463ms |
| /goals/eligible-reportees-for-cascade/5b39887b-6659-4d87-975f-508917131ea3 | 200 | 200 | 0 | 100.00% | 1641.16ms | 1990ms |
| /goals/get-goal-categories | 200 | 200 | 0 | 100.00% | 533.30ms | 662ms |
| /goals/get-objectives-akrs-and-quadrants/a915cb6e-0974-4fb8-9553-81f04ce7ca45 | 200 | 200 | 0 | 100.00% | 1624.13ms | 2014ms |
| /goals/detailed-goals/5b39887b-6659-4d87-975f-508917131ea3/0f75671e-a024-43bc-82aa-1f454f39ffd5 | 200 | 200 | 0 | 100.00% | 2654.23ms | 3035ms |
| /goals/cascade-json/0f75671e-a024-43bc-82aa-1f454f39ffd5/5b39887b-6659-4d87-975f-508917131ea3 | 200 | 200 | 0 | 100.00% | 1679.96ms | 2170ms |
| /periods/previous-current-next | 200 | 200 | 0 | 100.00% | 2218.20ms | 2492ms |
| /periods/5b39887b-6659-4d87-975f-508917131ea3/subperiods | 200 | 200 | 0 | 100.00% | 736.05ms | 895ms |
| /cycles/ | 200 | 200 | 0 | 100.00% | 593.67ms | 768ms | |

### Scenario 2 - Concurrency: 400

| Metric | Value |
|--------|-------|
| Concurrency | 400 |
| Total Requests | 7200 |
| Success Rate | 99.83% |
| RPS | 25.02 |
| Mean Latency | 1864.69ms |
| P95 Latency | 6741ms |
| Duration | 287.72s |

#### Endpoint Details:

| Endpoint | Requests | Success | Errors | Success Rate | Mean Latency | P95 Latency |
|----------|----------|---------|--------|--------------|--------------|-------------|
| | /auth/get-tenants | 400 | 400 | 0 | 100.00% | 1487.97ms | 1777ms |
| /user/me | 400 | 400 | 0 | 100.00% | 1311.97ms | 1604ms |
| /user/active-and-inactive-users | 400 | 400 | 0 | 100.00% | 3576.95ms | 4069ms |
| /user/get-user-skills/0f75671e-a024-43bc-82aa-1f454f39ffd5 | 400 | 400 | 0 | 100.00% | 1183.64ms | 1482ms |
| /user/team/0f75671e-a024-43bc-82aa-1f454f39ffd5 | 400 | 400 | 0 | 100.00% | 832.16ms | 1073ms |
| /user/chat-users/ | 400 | 400 | 0 | 100.00% | 3126.45ms | 3426ms |
| /user/all-users | 400 | 400 | 0 | 100.00% | 1381.79ms | 1620ms |
| /user/user-progress-details/0f75671e-a024-43bc-82aa-1f454f39ffd5/5b39887b-6659-4d87-975f-508917131ea3/a915cb6e-0974-4fb8-9553-81f04ce7ca45 | 400 | 388 | 12 | 97.00% | 13136.17ms | 15299ms |
| /goals/user-goals/5b39887b-6659-4d87-975f-508917131ea3/0f75671e-a024-43bc-82aa-1f454f39ffd5 | 400 | 400 | 0 | 100.00% | 790.29ms | 1039ms |
| /goals/manager-reportee/5b39887b-6659-4d87-975f-508917131ea3/0f75671e-a024-43bc-82aa-1f454f39ffd5 | 400 | 400 | 0 | 100.00% | 1330.72ms | 1561ms |
| /goals/eligible-reportees-for-cascade/5b39887b-6659-4d87-975f-508917131ea3 | 400 | 400 | 0 | 100.00% | 797.47ms | 1110ms |
| /goals/get-goal-categories | 400 | 400 | 0 | 100.00% | 244.69ms | 348ms |
| /goals/get-objectives-akrs-and-quadrants/a915cb6e-0974-4fb8-9553-81f04ce7ca45 | 400 | 400 | 0 | 100.00% | 709.41ms | 826ms |
| /goals/detailed-goals/5b39887b-6659-4d87-975f-508917131ea3/0f75671e-a024-43bc-82aa-1f454f39ffd5 | 400 | 400 | 0 | 100.00% | 1281.53ms | 1494ms |
| /goals/cascade-json/0f75671e-a024-43bc-82aa-1f454f39ffd5/5b39887b-6659-4d87-975f-508917131ea3 | 400 | 400 | 0 | 100.00% | 684.26ms | 1129ms |
| /periods/previous-current-next | 400 | 400 | 0 | 100.00% | 1050.71ms | 1207ms |
| /periods/5b39887b-6659-4d87-975f-508917131ea3/subperiods | 400 | 400 | 0 | 100.00% | 373.78ms | 468ms |
| /cycles/ | 400 | 400 | 0 | 100.00% | 264.41ms | 350ms | |

### Scenario 3 - Concurrency: 800

| Metric | Value |
|--------|-------|
| Concurrency | 800 |
| Total Requests | 14400 |
| Success Rate | 99.99% |
| RPS | 39.47 |
| Mean Latency | 1204.51ms |
| P95 Latency | 6504ms |
| Duration | 364.87s |

#### Endpoint Details:

| Endpoint | Requests | Success | Errors | Success Rate | Mean Latency | P95 Latency |
|----------|----------|---------|--------|--------------|--------------|-------------|
| | /auth/get-tenants | 800 | 800 | 0 | 100.00% | 1123.78ms | 1281ms |
| /user/me | 800 | 800 | 0 | 100.00% | 614.94ms | 798ms |
| /user/active-and-inactive-users | 800 | 799 | 1 | 99.88% | 1634.64ms | 1986ms |
| /user/get-user-skills/0f75671e-a024-43bc-82aa-1f454f39ffd5 | 800 | 800 | 0 | 100.00% | 692.50ms | 784ms |
| /user/team/0f75671e-a024-43bc-82aa-1f454f39ffd5 | 800 | 800 | 0 | 100.00% | 365.75ms | 578ms |
| /user/chat-users/ | 800 | 800 | 0 | 100.00% | 1629.35ms | 1950ms |
| /user/all-users | 800 | 800 | 0 | 100.00% | 696.26ms | 859ms |
| /user/user-progress-details/0f75671e-a024-43bc-82aa-1f454f39ffd5/5b39887b-6659-4d87-975f-508917131ea3/a915cb6e-0974-4fb8-9553-81f04ce7ca45 | 800 | 800 | 0 | 100.00% | 7161.16ms | 8082ms |
| /goals/user-goals/5b39887b-6659-4d87-975f-508917131ea3/0f75671e-a024-43bc-82aa-1f454f39ffd5 | 800 | 799 | 1 | 99.88% | 831.07ms | 1001ms |
| /goals/manager-reportee/5b39887b-6659-4d87-975f-508917131ea3/0f75671e-a024-43bc-82aa-1f454f39ffd5 | 800 | 800 | 0 | 100.00% | 1401.34ms | 1673ms |
| /goals/eligible-reportees-for-cascade/5b39887b-6659-4d87-975f-508917131ea3 | 800 | 800 | 0 | 100.00% | 728.80ms | 967ms |
| /goals/get-goal-categories | 800 | 800 | 0 | 100.00% | 264.61ms | 328ms |
| /goals/get-objectives-akrs-and-quadrants/a915cb6e-0974-4fb8-9553-81f04ce7ca45 | 800 | 800 | 0 | 100.00% | 753.03ms | 983ms |
| /goals/detailed-goals/5b39887b-6659-4d87-975f-508917131ea3/0f75671e-a024-43bc-82aa-1f454f39ffd5 | 800 | 800 | 0 | 100.00% | 1268.17ms | 1569ms |
| /goals/cascade-json/0f75671e-a024-43bc-82aa-1f454f39ffd5/5b39887b-6659-4d87-975f-508917131ea3 | 800 | 800 | 0 | 100.00% | 757.34ms | 1000ms |
| /periods/previous-current-next | 800 | 800 | 0 | 100.00% | 1104.57ms | 1317ms |
| /periods/5b39887b-6659-4d87-975f-508917131ea3/subperiods | 800 | 800 | 0 | 100.00% | 368.02ms | 460ms |
| /cycles/ | 800 | 800 | 0 | 100.00% | 285.89ms | 383ms | |

### Scenario 4 - Concurrency: 1600

| Metric | Value |
|--------|-------|
| Concurrency | 1600 |
| Total Requests | 28800 |
| Success Rate | 99.98% |
| RPS | 39.89 |
| Mean Latency | 1223.11ms |
| P95 Latency | 6579ms |
| Duration | 721.96s |

#### Endpoint Details:

| Endpoint | Requests | Success | Errors | Success Rate | Mean Latency | P95 Latency |
|----------|----------|---------|--------|--------------|--------------|-------------|
| | /auth/get-tenants | 1600 | 1600 | 0 | 100.00% | 1186.22ms | 1361ms |
| /user/me | 1600 | 1598 | 2 | 99.88% | 692.05ms | 842ms |
| /user/active-and-inactive-users | 1600 | 1600 | 0 | 100.00% | 1977.50ms | 2355ms |
| /user/get-user-skills/0f75671e-a024-43bc-82aa-1f454f39ffd5 | 1600 | 1600 | 0 | 100.00% | 695.16ms | 787ms |
| /user/team/0f75671e-a024-43bc-82aa-1f454f39ffd5 | 1600 | 1600 | 0 | 100.00% | 394.60ms | 478ms |
| /user/chat-users/ | 1600 | 1599 | 1 | 99.94% | 1636.77ms | 1912ms |
| /user/all-users | 1600 | 1599 | 1 | 99.94% | 685.91ms | 822ms |
| /user/user-progress-details/0f75671e-a024-43bc-82aa-1f454f39ffd5/5b39887b-6659-4d87-975f-508917131ea3/a915cb6e-0974-4fb8-9553-81f04ce7ca45 | 1600 | 1600 | 0 | 100.00% | 7115.71ms | 7908ms |
| /goals/user-goals/5b39887b-6659-4d87-975f-508917131ea3/0f75671e-a024-43bc-82aa-1f454f39ffd5 | 1600 | 1600 | 0 | 100.00% | 823.57ms | 996ms |
| /goals/manager-reportee/5b39887b-6659-4d87-975f-508917131ea3/0f75671e-a024-43bc-82aa-1f454f39ffd5 | 1600 | 1600 | 0 | 100.00% | 1389.56ms | 1596ms |
| /goals/eligible-reportees-for-cascade/5b39887b-6659-4d87-975f-508917131ea3 | 1600 | 1600 | 0 | 100.00% | 740.39ms | 900ms |
| /goals/get-goal-categories | 1600 | 1600 | 0 | 100.00% | 230.70ms | 297ms |
| /goals/get-objectives-akrs-and-quadrants/a915cb6e-0974-4fb8-9553-81f04ce7ca45 | 1600 | 1600 | 0 | 100.00% | 752.53ms | 916ms |
| /goals/detailed-goals/5b39887b-6659-4d87-975f-508917131ea3/0f75671e-a024-43bc-82aa-1f454f39ffd5 | 1600 | 1599 | 1 | 99.94% | 1174.82ms | 1499ms |
| /goals/cascade-json/0f75671e-a024-43bc-82aa-1f454f39ffd5/5b39887b-6659-4d87-975f-508917131ea3 | 1600 | 1600 | 0 | 100.00% | 771.98ms | 920ms |
| /periods/previous-current-next | 1600 | 1600 | 0 | 100.00% | 1151.08ms | 1321ms |
| /periods/5b39887b-6659-4d87-975f-508917131ea3/subperiods | 1600 | 1600 | 0 | 100.00% | 354.65ms | 449ms |
| /cycles/ | 1600 | 1600 | 0 | 100.00% | 242.78ms | 324ms | |

### Scenario 5 - Concurrency: 3200

| Metric | Value |
|--------|-------|
| Concurrency | 3200 |
| Total Requests | 57600 |
| Success Rate | 99.99% |
| RPS | 42.36 |
| Mean Latency | 1163.77ms |
| P95 Latency | 6268ms |
| Duration | 1359.68s |

#### Endpoint Details:

| Endpoint | Requests | Success | Errors | Success Rate | Mean Latency | P95 Latency |
|----------|----------|---------|--------|--------------|--------------|-------------|
| | /auth/get-tenants | 3200 | 3200 | 0 | 100.00% | 970.12ms | 1161ms |
| /user/me | 3200 | 3199 | 1 | 99.97% | 624.30ms | 751ms |
| /user/active-and-inactive-users | 3200 | 3199 | 1 | 99.97% | 1730.87ms | 2021ms |
| /user/get-user-skills/0f75671e-a024-43bc-82aa-1f454f39ffd5 | 3200 | 3200 | 0 | 100.00% | 685.86ms | 715ms |
| /user/team/0f75671e-a024-43bc-82aa-1f454f39ffd5 | 3200 | 3200 | 0 | 100.00% | 350.67ms | 431ms |
| /user/chat-users/ | 3200 | 3200 | 0 | 100.00% | 1485.87ms | 1735ms |
| /user/all-users | 3200 | 3200 | 0 | 100.00% | 666.00ms | 797ms |
| /user/user-progress-details/0f75671e-a024-43bc-82aa-1f454f39ffd5/5b39887b-6659-4d87-975f-508917131ea3/a915cb6e-0974-4fb8-9553-81f04ce7ca45 | 3200 | 3200 | 0 | 100.00% | 6687.18ms | 7481ms |
| /goals/user-goals/5b39887b-6659-4d87-975f-508917131ea3/0f75671e-a024-43bc-82aa-1f454f39ffd5 | 3200 | 3200 | 0 | 100.00% | 869.45ms | 1027ms |
| /goals/manager-reportee/5b39887b-6659-4d87-975f-508917131ea3/0f75671e-a024-43bc-82aa-1f454f39ffd5 | 3200 | 3198 | 2 | 99.94% | 1428.48ms | 1646ms |
| /goals/eligible-reportees-for-cascade/5b39887b-6659-4d87-975f-508917131ea3 | 3200 | 3199 | 1 | 99.97% | 771.70ms | 944ms |
| /goals/get-goal-categories | 3200 | 3200 | 0 | 100.00% | 203.48ms | 250ms |
| /goals/get-objectives-akrs-and-quadrants/a915cb6e-0974-4fb8-9553-81f04ce7ca45 | 3200 | 3200 | 0 | 100.00% | 725.57ms | 757ms |
| /goals/detailed-goals/5b39887b-6659-4d87-975f-508917131ea3/0f75671e-a024-43bc-82aa-1f454f39ffd5 | 3200 | 3200 | 0 | 100.00% | 1285.87ms | 1516ms |
| /goals/cascade-json/0f75671e-a024-43bc-82aa-1f454f39ffd5/5b39887b-6659-4d87-975f-508917131ea3 | 3200 | 3200 | 0 | 100.00% | 782.67ms | 928ms |
| /periods/previous-current-next | 3200 | 3198 | 2 | 99.94% | 1092.54ms | 1256ms |
| /periods/5b39887b-6659-4d87-975f-508917131ea3/subperiods | 3200 | 3200 | 0 | 100.00% | 345.47ms | 435ms |
| /cycles/ | 3200 | 3200 | 0 | 100.00% | 241.80ms | 315ms | |

### Scenario 6 - Concurrency: 6400

| Metric | Value |
|--------|-------|
| Concurrency | 6400 |
| Total Requests | 115200 |
| Success Rate | 99.99% |
| RPS | 39.53 |
| Mean Latency | 1255.36ms |
| P95 Latency | 6906ms |
| Duration | 2914.51s |

#### Endpoint Details:

| Endpoint | Requests | Success | Errors | Success Rate | Mean Latency | P95 Latency |
|----------|----------|---------|--------|--------------|--------------|-------------|
| | /auth/get-tenants | 6400 | 6400 | 0 | 100.00% | 1001.17ms | 1180ms |
| /user/me | 6400 | 6400 | 0 | 100.00% | 657.05ms | 804ms |
| /user/active-and-inactive-users | 6400 | 6399 | 1 | 99.98% | 1850.66ms | 2186ms |
| /user/get-user-skills/0f75671e-a024-43bc-82aa-1f454f39ffd5 | 6400 | 6400 | 0 | 100.00% | 727.67ms | 747ms |
| /user/team/0f75671e-a024-43bc-82aa-1f454f39ffd5 | 6400 | 6400 | 0 | 100.00% | 374.96ms | 457ms |
| /user/chat-users/ | 6400 | 6398 | 2 | 99.97% | 1612.19ms | 1840ms |
| /user/all-users | 6400 | 6396 | 4 | 99.94% | 711.96ms | 863ms |
| /user/user-progress-details/0f75671e-a024-43bc-82aa-1f454f39ffd5/5b39887b-6659-4d87-975f-508917131ea3/a915cb6e-0974-4fb8-9553-81f04ce7ca45 | 6400 | 6399 | 1 | 99.98% | 7408.00ms | 8188ms |
| /goals/user-goals/5b39887b-6659-4d87-975f-508917131ea3/0f75671e-a024-43bc-82aa-1f454f39ffd5 | 6400 | 6400 | 0 | 100.00% | 922.85ms | 1087ms |
| /goals/manager-reportee/5b39887b-6659-4d87-975f-508917131ea3/0f75671e-a024-43bc-82aa-1f454f39ffd5 | 6400 | 6398 | 2 | 99.97% | 1543.89ms | 1778ms |
| /goals/eligible-reportees-for-cascade/5b39887b-6659-4d87-975f-508917131ea3 | 6400 | 6399 | 1 | 99.98% | 848.70ms | 1015ms |
| /goals/get-goal-categories | 6400 | 6400 | 0 | 100.00% | 234.59ms | 303ms |
| /goals/get-objectives-akrs-and-quadrants/a915cb6e-0974-4fb8-9553-81f04ce7ca45 | 6400 | 6400 | 0 | 100.00% | 713.42ms | 826ms |
| /goals/detailed-goals/5b39887b-6659-4d87-975f-508917131ea3/0f75671e-a024-43bc-82aa-1f454f39ffd5 | 6400 | 6398 | 2 | 99.97% | 1385.74ms | 1589ms |
| /goals/cascade-json/0f75671e-a024-43bc-82aa-1f454f39ffd5/5b39887b-6659-4d87-975f-508917131ea3 | 6400 | 6400 | 0 | 100.00% | 824.33ms | 996ms |
| /periods/previous-current-next | 6400 | 6397 | 3 | 99.95% | 1151.03ms | 1330ms |
| /periods/5b39887b-6659-4d87-975f-508917131ea3/subperiods | 6400 | 6400 | 0 | 100.00% | 361.73ms | 448ms |
| /cycles/ | 6400 | 6400 | 0 | 100.00% | 266.60ms | 344ms | |

### Scenario 7 - Concurrency: 10000

| Metric | Value |
|--------|-------|
| Concurrency | 10000 |
| Total Requests | 180000 |
| Success Rate | 99.99% |
| RPS | 40.15 |
| Mean Latency | 1238.65ms |
| P95 Latency | 6894ms |
| Duration | 4483.73s |

#### Endpoint Details:

| Endpoint | Requests | Success | Errors | Success Rate | Mean Latency | P95 Latency |
|----------|----------|---------|--------|--------------|--------------|-------------|
| | /auth/get-tenants | 10000 | 10000 | 0 | 100.00% | 1017.00ms | 1214ms |
| /user/me | 10000 | 9997 | 3 | 99.97% | 658.15ms | 794ms |
| /user/active-and-inactive-users | 10000 | 10000 | 0 | 100.00% | 1859.38ms | 2153ms |
| /user/get-user-skills/0f75671e-a024-43bc-82aa-1f454f39ffd5 | 10000 | 10000 | 0 | 100.00% | 688.50ms | 715ms |
| /user/team/0f75671e-a024-43bc-82aa-1f454f39ffd5 | 10000 | 10000 | 0 | 100.00% | 376.26ms | 462ms |
| /user/chat-users/ | 10000 | 9997 | 3 | 99.97% | 1627.66ms | 1878ms |
| /user/all-users | 10000 | 9996 | 4 | 99.96% | 725.57ms | 882ms |
| /user/user-progress-details/0f75671e-a024-43bc-82aa-1f454f39ffd5/5b39887b-6659-4d87-975f-508917131ea3/a915cb6e-0974-4fb8-9553-81f04ce7ca45 | 10000 | 9998 | 2 | 99.98% | 7345.99ms | 8124ms |
| /goals/user-goals/5b39887b-6659-4d87-975f-508917131ea3/0f75671e-a024-43bc-82aa-1f454f39ffd5 | 10000 | 9998 | 2 | 99.98% | 879.27ms | 1055ms |
| /goals/manager-reportee/5b39887b-6659-4d87-975f-508917131ea3/0f75671e-a024-43bc-82aa-1f454f39ffd5 | 10000 | 9997 | 3 | 99.97% | 1468.13ms | 1676ms |
| /goals/eligible-reportees-for-cascade/5b39887b-6659-4d87-975f-508917131ea3 | 10000 | 10000 | 0 | 100.00% | 795.00ms | 963ms |
| /goals/get-goal-categories | 10000 | 10000 | 0 | 100.00% | 220.19ms | 289ms |
| /goals/get-objectives-akrs-and-quadrants/a915cb6e-0974-4fb8-9553-81f04ce7ca45 | 10000 | 10000 | 0 | 100.00% | 692.89ms | 723ms |
| /goals/detailed-goals/5b39887b-6659-4d87-975f-508917131ea3/0f75671e-a024-43bc-82aa-1f454f39ffd5 | 10000 | 9997 | 3 | 99.97% | 1374.97ms | 1605ms |
| /goals/cascade-json/0f75671e-a024-43bc-82aa-1f454f39ffd5/5b39887b-6659-4d87-975f-508917131ea3 | 10000 | 9999 | 1 | 99.99% | 798.87ms | 949ms |
| /periods/previous-current-next | 10000 | 9999 | 1 | 99.99% | 1144.19ms | 1316ms |
| /periods/5b39887b-6659-4d87-975f-508917131ea3/subperiods | 10000 | 9999 | 1 | 99.99% | 369.14ms | 453ms |
| /cycles/ | 10000 | 10000 | 0 | 100.00% | 254.48ms | 336ms | |


## 6. Recommendations

Based on the scaling analysis:

1. **Optimal Operating Range:** 5000 - 10000 concurrent connections provide the best balance of throughput and latency
2. **Performance Degradation:** Monitor closely beyond 3.0776072251884448 concurrent connections
3. **Resource Planning:** System shows System maintains stable performance across all tested concurrency levels with effective auto-scalingx scaling capability from baseline to peak performance
4. **Bottleneck Analysis:** Based on the scaling analysis:

1. **Optimal Operating Range:** 5000 - 10000 concurrent connections provide the best balance of throughput and latency
2. **Performance Degradation:** Monitor closely beyond 10000 concurrent connections
3. **Resource Planning:** System shows 3.1x scaling capability from baseline to peak performance
4. **Bottleneck Analysis:** System maintains stable performance across all tested concurrency levels with effective auto-scaling

