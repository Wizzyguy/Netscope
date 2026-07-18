# Collector Module

## Purpose

The Collector module is responsible for discovering Linux processes, monitoring
their network activity, gathering CPU and memory information, and producing the
raw data consumed by the Engine layer.

---

# Responsibilities

- Discover running processes
- Read process metadata
- Discover socket ownership
- Parse /proc filesystem
- Parse network statistics
- Track per-process bandwidth
- Track CPU usage
- Track memory usage
- Capture active TCP connections
- Capture keyboard input

---

# Collector Architecture

Process Discovery
        │
        ▼
Socket Discovery
        │
        ▼
Network Usage
        │
        ▼
CPU Reader
        │
        ▼
Memory Reader
        │
        ▼
Connection Discovery
        │
        ▼
Engine

---

# Current Modules

| Module | Responsibility |
|---------|----------------|
| process_discovery.rs | Discover running processes |
| socket_discovery.rs | Map sockets to PIDs |
| process_usage.rs | Per-process bandwidth |
| cpu_tracker.rs | CPU tracking |
| memory.rs | Memory usage |
| tcp_parser.rs | TCP connection parsing |
| connection_builder.rs | Build live connections |
| keyboard.rs | Keyboard events |

---

# Data Flow

1. Scan `/proc`
2. Discover active processes
3. Discover socket inodes
4. Match sockets to processes
5. Read `/proc/net`
6. Compute bandwidth
7. Return ProcessSession objects to the Engine

---

# Performance Notes

Current refresh interval:

100 ms

Current design:

Polling

Future improvements:

- eBPF collector
- Netlink sockets
- Async collection
- Incremental updates

---

# Future Improvements

- eBPF support
- IPv6 optimization
- UDP monitoring
- DNS monitoring
- Container awareness
- Namespace support
