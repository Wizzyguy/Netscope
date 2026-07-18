# NetScope

## Africa Deep Tech Challenge 2026

---

# Problem Statement

Linux users often lack an integrated tool that correlates process activity, bandwidth usage, CPU utilization, memory consumption, and active network connections into a single interface.

Existing tools typically focus on only one aspect of system monitoring.

NetScope aims to bridge this gap.

---

# Solution

NetScope is a lightweight observability platform that continuously monitors Linux processes and displays:

- Live bandwidth
- CPU usage
- Memory usage
- Session statistics
- Active network connections

through an interactive terminal dashboard.

---

# Objectives

- Improve network visibility
- Simplify process monitoring
- Provide actionable insights
- Prepare for AI-assisted analysis

---

# Technical Stack

Programming Language

Rust

Libraries

- Ratatui
- Crossterm

Operating System

Linux

Future Technologies

- SQLite
- eBPF
- VirusTotal API

---

# System Architecture

```
Collector
     │
     ▼
Engine
     │
     ▼
User Interface
```

---

# Current Features

- Dashboard
- Connection Viewer
- Process Inspector
- Session Tracking
- Search
- Sorting
- Workspace Navigation

---

# Planned Features

- Security Analysis
- AI Recommendations
- Historical Database
- Notifications
- Reporting
- Export Functions

---

# Challenges

Current development challenges include:

- Mapping sockets to processes
- Accurate CPU percentage calculation
- Efficient refresh scheduling
- Maintaining a responsive terminal interface

---

# Future Work

- SQLite persistence
- Notification engine
- AI-assisted diagnostics
- eBPF integration
- Interactive graphs
- Multi-platform support

---

# Current Status

Version

0.4

Development Stage

Active Development

---

# Repository

https://github.com/Wizzyguy/Netscope
