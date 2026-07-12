use std::{collections::HashMap, time::Instant};

pub struct CpuTracker {
    previous: HashMap<u32, (u64, Instant)>,
}

impl CpuTracker {
    pub fn new() -> Self {
        Self {
            previous: HashMap::new(),
        }
    }

    pub fn calculate(&mut self, pid: u32, cpu_ticks: u64) -> f64 {
        let now = Instant::now();

        if let Some((old_ticks, old_time)) = self.previous.get(&pid) {
            let tick_delta = cpu_ticks.saturating_sub(*old_ticks);

            let elapsed = now.duration_since(*old_time).as_secs_f64();

            self.previous.insert(pid, (cpu_ticks, now));

            if elapsed == 0.0 {
                return 0.0;
            }

            // Placeholder calculation.
            // We'll normalize using system clock ticks later.
            return tick_delta as f64 / elapsed;
        }

        self.previous.insert(pid, (cpu_ticks, now));

        0.0
    }

    pub fn cleanup(&mut self, active_pids: &[u32]) {
        self.previous.retain(|pid, _| active_pids.contains(pid));
    }
}
