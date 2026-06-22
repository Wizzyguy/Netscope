# Collector Sampling Algorithm

Goal:
Estimate process bandwidth over time.

Process:

1. Discover active PIDs
2. Resolve process names
3. Discover owned sockets
4. Capture current counters
5. Compare with previous snapshot
6. Compute upload/download delta
7. Emit usage record

Formula:

usage = current - previous

Loop:

sample
wait
sample
calculate
store
repeat
