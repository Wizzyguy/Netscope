# NetScope Architecture (Phase 0)

## Overview
NetScope is a Linux desktop application that tracks per-process network usage in real time.

## High-Level Design

UI (Tauri)
  ↓
Analytics Layer
  ↓
Storage Layer (SQLite)
  ↓
Collector Layer (/proc + sockets)

## Core Principle
Keep collection lightweight and offline-first.

## Future Expansion
- eBPF-based traffic attribution
- AI-based usage insights
- Predictive bandwidth analysis
