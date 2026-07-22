use std::{
    collections::HashMap,
    time::Instant,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProcessState {
    Started,
    Active,
    Idle,
    Exited,
}

#[derive(Clone)]
pub struct LifecycleRecord {
    pub pid: u32,
    pub name: String,

    pub state: ProcessState,

    pub first_seen: Instant,
    pub last_seen: Instant,

    pub active_seconds: u64,
    pub idle_seconds: u64,
}

pub struct LifecycleEngine {
    processes: HashMap<u32, LifecycleRecord>,
}

impl LifecycleEngine {
    pub fn new() -> Self {
        Self {
            processes: HashMap::new(),
        }
    }

    //--------------------------------------------------
    // Update
    //--------------------------------------------------

    pub fn update(
        &mut self,
        pid: u32,
        name: String,
        active: bool,
    ) -> Option<String> {
        let now = Instant::now();

        //--------------------------------------------------
        // Existing Process
        //--------------------------------------------------

        if let Some(record) = self.processes.get_mut(&pid) {
            let elapsed =
                now.duration_since(record.last_seen).as_secs();

            record.last_seen = now;

            if active {
                record.active_seconds += elapsed;

                if record.state != ProcessState::Active {
                    record.state = ProcessState::Active;
                    return Some("active".to_string());
                }
            } else {
                record.idle_seconds += elapsed;

                if record.state != ProcessState::Idle {
                    record.state = ProcessState::Idle;
                    return Some("idle".to_string());
                }
            }

            return None;
        }

        //--------------------------------------------------
        // New Process
        //--------------------------------------------------

        self.processes.insert(
            pid,
            LifecycleRecord {
                pid,
                name,

                state: ProcessState::Started,

                first_seen: now,
                last_seen: now,

                active_seconds: 0,
                idle_seconds: 0,
            },
        );

        Some("started".to_string())
    }

    //--------------------------------------------------
    // Detect Exited Processes
    //--------------------------------------------------

    pub fn cleanup(
        &mut self,
        active_pids: &[u32],
    ) -> Vec<(u32, String)> {

        let mut exited = Vec::new();

        for record in self.processes.values_mut() {

            if !active_pids.contains(&record.pid)
                && record.state != ProcessState::Exited
            {
                record.state = ProcessState::Exited;

                exited.push((
                    record.pid,
                    record.name.clone(),
                ));
            }
        }

        exited
    }

    //--------------------------------------------------
    // Records
    //--------------------------------------------------

    pub fn records(
        &self,
    ) -> Vec<LifecycleRecord> {

        self.processes
            .values()
            .cloned()
            .collect()

    }

    //--------------------------------------------------
    // Clear
    //--------------------------------------------------

    pub fn clear(
        &mut self,
    ) {
        self.processes.clear();
    }
}
