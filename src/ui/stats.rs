use std::time::{SystemTime, UNIX_EPOCH};

pub struct DashboardStats {
    pub processes: usize,
    pub active: usize,
    pub total_rx: u64,
    pub total_tx: u64,
}

impl DashboardStats {
    pub fn current_time() -> String {
        let secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let seconds = secs % 60;
        let minutes = (secs / 60) % 60;
        let hours = (secs / 3600) % 24;

        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }
}
