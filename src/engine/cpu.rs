use std::collections::HashMap;

use crate::collector::read_process_cpu;

pub struct CpuEngine {
    previous: HashMap<u32, u64>,
    values: HashMap<u32, f32>,
}

impl CpuEngine {
    pub fn new() -> Self {
        Self {
            previous: HashMap::new(),
            values: HashMap::new(),
        }
    }

    //--------------------------------------------------
    // Update
    //--------------------------------------------------

    pub fn update(
        &mut self,
        pid: u32,
    ) {
        let current =
            read_process_cpu(pid).unwrap_or(0);

        let previous =
            self.previous
                .get(&pid)
                .copied()
                .unwrap_or(current);

        let delta =
            current.saturating_sub(previous);

        // Temporary approximation
        let cpu =
            delta as f32 / 10.0;

        self.previous.insert(pid, current);

        self.values.insert(pid, cpu);
    }

    //--------------------------------------------------
    // Query
    //--------------------------------------------------

    pub fn value(
        &self,
        pid: u32,
    ) -> f32 {
        *self.values
            .get(&pid)
            .unwrap_or(&0.0)
    }

    //--------------------------------------------------
    // Cleanup
    //--------------------------------------------------

    pub fn cleanup(
        &mut self,
        active: &[u32],
    ) {
        self.previous
            .retain(|pid, _| active.contains(pid));

        self.values
            .retain(|pid, _| active.contains(pid));
    }

    //--------------------------------------------------
    // Reset
    //--------------------------------------------------

    pub fn reset(&mut self) {
        self.previous.clear();
        self.values.clear();
    }
}
