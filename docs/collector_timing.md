# Collector Timing

Decision:
Sample once per second.

Reason:
- low overhead
- good responsiveness
- simpler persistence

Future:
Adaptive sampling may be added.

Timing:

collect
wait 1 second
collect
calculate
store
repeat
