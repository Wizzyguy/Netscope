# Collector Modules

## process_discovery
Discover running processes.

Input:
- /proc

Output:
- pid
- process_name

---

## socket_mapper
Map sockets to processes.

Input:
- /proc/[pid]/fd
- /proc/net

Output:
- process → sockets

---

## sampler
Capture periodic measurements.

Output:
- snapshots

---

## usage_calculator
Compute bandwidth usage.

Formula:

usage = current - previous
