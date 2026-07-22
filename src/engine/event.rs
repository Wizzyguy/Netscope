use std::time::SystemTime;

#[derive(Clone)]
pub struct ProcessEvent {
    pub timestamp: SystemTime,
    pub pid: u32,
    pub process: String,
    pub message: String,
}

pub struct EventEngine {
    events: Vec<ProcessEvent>,
}

impl EventEngine {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
        }
    }

    //--------------------------------------------------
    // Add Event
    //--------------------------------------------------

    pub fn add(
        &mut self,
        pid: u32,
        process: String,
        message: String,
    ) {
        self.events.push(ProcessEvent {
            timestamp: SystemTime::now(),
            pid,
            process,
            message,
        });

        //--------------------------------------------------
        // Keep only latest 1000 events
        //--------------------------------------------------

        if self.events.len() > 1000 {
            self.events.remove(0);
        }
    }

    //--------------------------------------------------
    // Read Events
    //--------------------------------------------------

    pub fn events(&self) -> &[ProcessEvent] {
        &self.events
    }

    //--------------------------------------------------
    // Clear
    //--------------------------------------------------

    pub fn clear(&mut self) {
        self.events.clear();
    }
}
