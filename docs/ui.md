# User Interface

## Purpose

The UI module is responsible for presenting processed information to the user
through an interactive terminal interface built with Ratatui.

The UI never performs data collection. It only renders information received from
the Engine.

---

# Responsibilities

- Render Dashboard
- Render Connections Workspace
- Render Security Workspace
- Render Analytics Workspace
- Display Header
- Display Footer
- Display Process Inspector
- Handle Navigation
- Display Search Bar

---

# Layout

```
+-------------------------------------------------------+
| Header                                                |
+-------------------------------------------------------+
| Workspace Content                                     |
|                                                       |
|                                                       |
+-------------------------------------------------------+
| Footer                                                |
+-------------------------------------------------------+
```

---

# Current Workspaces

## Dashboard

Displays

- Live download speed
- Live upload speed
- Total bandwidth
- CPU usage
- Memory usage
- Session usage

---

## Connections

Displays

- Active TCP connections
- Remote hosts
- Ports
- Protocol
- Connection state

---

## Security

Reserved for

- VirusTotal
- Digital Signatures
- SHA256
- Reputation
- Risk Analysis

---

## Analytics

Reserved for

- Charts
- Trends
- Statistics
- AI Recommendations

---

# Keyboard Shortcuts

| Key | Action |
|------|---------|
| ← | Previous workspace |
| → | Next workspace |
| ↑ | Previous process |
| ↓ | Next process |
| / | Search |
| R | Reset Session |
| Q | Quit |

---

# Future Improvements

- Mouse support
- Theme system
- Resizable panels
- Graph widgets
- Popup windows
- Notifications
