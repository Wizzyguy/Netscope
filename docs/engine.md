# Engine Module

## Purpose

The Engine module transforms raw information collected from the Collector into
structured data that can be displayed by the user interface.

It acts as the central processing layer of NetScope.

---

# Responsibilities

- Process bandwidth calculations
- Track live bandwidth speed
- Maintain cumulative totals
- Track session usage
- Compute CPU utilization
- Track memory consumption
- Build dashboard rows
- Manage live network connections
- Provide processed data to the UI

---

# Architecture

```
Collector
     │
     ▼
Engine
 ├── Bandwidth Engine
 ├── CPU Engine
 ├── Memory Engine
 ├── Session Engine
 └── Connection Engine
     │
     ▼
UI
```

---

# Engine Components

## Bandwidth Engine

Responsibilities

- Live download speed
- Live upload speed
- Total downloaded
- Total uploaded

Future

- Historical bandwidth
- Daily statistics

---

## CPU Engine

Responsibilities

- Read CPU ticks
- Calculate process CPU percentage
- Remove inactive processes

Future

- Multi-core awareness
- Accurate utilization timing

---

## Memory Engine

Responsibilities

- Read process memory
- Store current memory usage
- Cleanup terminated processes

Future

- Memory history
- Peak memory tracking

---

## Session Engine

Responsibilities

- Session download
- Session upload
- Session totals
- Reset sessions

Future

- Daily sessions
- Weekly sessions
- Monthly sessions

---

## Connection Engine

Responsibilities

- Maintain active TCP connections
- Track connection count
- Track unique processes
- Refresh connection list

Future

- UDP
- IPv6
- DNS
- TLS metadata

---

# Engine Data Flow

1. Collector discovers processes
2. Collector reads bandwidth
3. Engine updates bandwidth totals
4. Engine updates CPU
5. Engine updates memory
6. Engine updates sessions
7. Engine refreshes connections
8. Engine builds dashboard cache
9. UI renders dashboard

---

# Performance

Current refresh interval

100 milliseconds

Current strategy

Polling

Future strategy

- Async updates
- Event-driven refresh
- eBPF integration

---

# Future Improvements

- Historical statistics
- SQLite integration
- AI analysis
- Alert engine
- Predictive analytics
- Resource optimization
