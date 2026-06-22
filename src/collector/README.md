# Collector

Purpose:
Collect and attribute network usage per process.

Responsibilities:

1. Discover active processes
2. Read process metadata
3. Discover socket ownership
4. Sample network activity
5. Produce usage records

Inputs:
- /proc
- /proc/net
- sockets

Outputs:

PID
Process Name
Upload Bytes
Download Bytes
Timestamp
