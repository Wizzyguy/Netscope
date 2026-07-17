use std::collections::HashMap;

use crate::collector::read_process_memory;

pub struct MemoryEngine {
    values: HashMap<u32, u64>,
}

impl MemoryEngine {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    //----------------------------------------------------
    // Update
    //----------------------------------------------------

    pub fn update(
        &mut self,
        pid: u32,
    ) {
        let memory =
            read_process_memory(pid)
                .unwrap_or(0);

        self.values.insert(
            pid,
            memory,
        );
    }

    //----------------------------------------------------
    // Query
    //----------------------------------------------------

    pub fn value(
        &self,
        pid: u32,
    ) -> u64 {
        *self
            .values
            .get(&pid)
            .unwrap_or(&0)
    }

    //----------------------------------------------------
    // Cleanup
    //----------------------------------------------------

    pub fn cleanup(
        &mut self,
        active: &[u32],
    ) {
        self.values
            .retain(|pid, _| active.contains(pid));
    }

    //----------------------------------------------------
    // Reset
    //----------------------------------------------------

    pub fn reset(
        &mut self,
    ) {
        self.values.clear();
    }
}
