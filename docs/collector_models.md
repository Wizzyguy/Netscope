# Collector Data Models

## Snapshot

Represents one observation cycle.

Fields:
- timestamp
- pid
- process_name
- rx_bytes
- tx_bytes

---

## ProcessUsage

Represents calculated traffic.

Fields:
- pid
- process_name
- upload_bytes
- download_bytes
- total_bytes
- timestamp

---

## CollectorState

Tracks collector memory.

Fields:
- previous_snapshot
- current_snapshot
- refresh_interval
