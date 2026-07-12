use std::collections::HashMap;

/// Tracks previous RX/TX totals for every process.
///
/// Used to calculate live throughput (bytes per refresh).
pub struct ThroughputTracker {
    previous: HashMap<u32, (u64, u64)>,
}

impl ThroughputTracker {
    /// Create a new throughput tracker.
    pub fn new() -> Self {
        Self {
            previous: HashMap::new(),
        }
    }

    /// Calculate the current throughput for a process.
    ///
    /// Returns:
    /// (download_speed, upload_speed)
    pub fn calculate(&mut self, pid: u32, rx: u64, tx: u64) -> (u64, u64) {
        let (old_rx, old_tx) = self.previous.get(&pid).copied().unwrap_or((rx, tx));

        let rx_speed = rx.saturating_sub(old_rx);

        let tx_speed = tx.saturating_sub(old_tx);

        self.previous.insert(pid, (rx, tx));

        (rx_speed, tx_speed)
    }

    /// Remove stale processes.
    ///
    /// Prevents memory growth when processes terminate.
    pub fn cleanup(&mut self, active_pids: &[u32]) {
        self.previous.retain(|pid, _| active_pids.contains(pid));
    }

    /// Clears all stored history.
    pub fn reset(&mut self) {
        self.previous.clear();
    }

    /// Number of tracked processes.
    pub fn tracked_processes(&self) -> usize {
        self.previous.len()
    }
}
