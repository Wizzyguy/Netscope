# Database

## Purpose

NetScope currently operates entirely in memory.

No persistent storage is used.

Future versions will introduce SQLite for long-term storage.

---

# Planned Storage

SQLite

---

# Planned Tables

## sessions

Stores

- PID
- Process
- Download
- Upload
- Timestamp

---

## alerts

Stores

- Rule
- Threshold
- Trigger Time

---

## history

Stores

- Daily usage
- Weekly usage
- Monthly usage

---

# Future Features

- Automatic cleanup
- Export CSV
- Export JSON
- Import history
- Backup database
