# NetScope

> A modern Linux network observability platform for monitoring per-process network usage, resource consumption, and security insights.

---

## Overview

NetScope is a lightweight terminal-based network observability application built in Rust.

It helps users understand exactly **which Linux processes are consuming network bandwidth**, how much CPU and memory they are using, and what network connections they have established.

Unlike traditional bandwidth monitors that only report interface-wide traffic, NetScope attributes network activity directly to the responsible process.

---

## Problem

Linux provides excellent networking tools, but answering questions like:

- Which application is using my bandwidth?
- Which process opened this connection?
- Which application is uploading data?
- Which program is consuming the most memory?

often requires multiple terminal commands and manual investigation.

NetScope brings these answers together into a single interactive dashboard.

---

## Features

### Dashboard

- Live Download Speed
- Live Upload Speed
- Total Session Usage
- CPU Usage
- Memory Usage
- Process Inspector
- Search
- Sorting
- Session Tracking

---

### Connections Workspace

- Active TCP Connections
- Remote Hosts
- Protocol
- Connection State
- Port Numbers

---

### Security Workspace *(In Development)*

- SHA256 Hashes
- Digital Signatures
- VirusTotal Integration
- Reputation Analysis
- Country Detection
- Risk Scoring

---

### Analytics Workspace *(In Development)*

- Usage Trends
- Historical Reports
- Daily Statistics
- Weekly Statistics
- Monthly Statistics
- AI Insights

---

## Architecture

```
Linux Kernel
      │
      ▼
Collector
      │
      ▼
Engine
      │
      ▼
Terminal UI
```

---

## Technology Stack

- Rust
- Ratatui
- Crossterm
- Linux /proc Filesystem

Future

- SQLite
- eBPF
- AI Engine

---

## Project Structure

```
src/
 ├── collector/
 ├── engine/
 ├── ui/
 └── main.rs

docs/

assets/

tests/
```

---

## Roadmap

### Version 0.4

- Dashboard
- Connections Workspace
- Session Tracking

### Version 0.5

- Security Workspace
- Notification Engine
- SQLite Integration

### Version 0.6

- Analytics Workspace
- AI Recommendation Engine

### Version 1.0

- Production Release

---

## Keyboard Shortcuts

| Key | Action |
|------|---------|
| ← → | Switch Workspace |
| ↑ ↓ | Select Process |
| / | Search |
| D | Sort Download |
| U | Sort Upload |
| N | Sort Name |
| P | Sort PID |
| R | Reset Session |
| Q | Quit |

---

## Current Status

Version

```
0.4
```

Status

```
Active Development
```

---

## License

MIT License
