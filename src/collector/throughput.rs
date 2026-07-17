use std::collections::HashMap;

/// Stores all runtime statistics for a process.
#[derive(Clone, Copy)]
struct ThroughputState {
    previous_rx: u64,
    previous_tx: u64,

    session_rx: u64,
    session_tx: u64,
}

/// Tracks live throughput and cumulative session usage.
pub struct ThroughputTracker {
    processes: HashMap<u32, ThroughputState>,
}

impl ThroughputTracker {
    /// Create a new tracker.
    pub fn new() -> Self {
        Self {
            processes: HashMap::new(),
        }
    }

    /// Calculates:
    ///
    /// (RX/s, TX/s, Session RX, Session TX)
    pub fn calculate(
        &mut self,
        pid: u32,
        rx_total: u64,
        tx_total: u64,
    ) -> (u64, u64, u64, u64) {
        let state = self.processes.entry(pid).or_insert(
            ThroughputState {
                previous_rx: rx_total,
                previous_tx: tx_total,

                session_rx: 0,
                session_tx: 0,
            },
        );

        //----------------------------------------------------
        // Live throughput
        //----------------------------------------------------

        let rx_speed = rx_total.saturating_sub(state.previous_rx);

        let tx_speed = tx_total.saturating_sub(state.previous_tx);

        //----------------------------------------------------
        // Update session totals
        //----------------------------------------------------

        state.session_rx += rx_speed;
        state.session_tx += tx_speed;

        //----------------------------------------------------
        // Store latest counters
        //----------------------------------------------------

        state.previous_rx = rx_total;
        state.previous_tx = tx_total;

        (
            rx_speed,
            tx_speed,
            state.session_rx,
            state.session_tx,
        )
    }

    /// Remove dead processes.
    pub fn cleanup(
        &mut self,
        active_pids: &[u32],
    ) {
        self.processes
            .retain(|pid, _| active_pids.contains(pid));
    }

    /// Reset every accumulated session total.
    pub fn reset(&mut self) {
        self.processes.clear();
    }

    /// Reset a single process.
    pub fn reset_process(&mut self, pid: u32) {
        self.processes.remove(&pid);
    }

    /// Number of tracked processes.
    pub fn tracked_processes(&self) -> usize {
        self.processes.len()
    }

    /// Total cumulative download since NetScope started.
    pub fn total_session_rx(&self) -> u64 {
        self.processes
            .values()
            .map(|p| p.session_rx)
            .sum()
    }

    /// Total cumulative upload since NetScope started.
    pub fn total_session_tx(&self) -> u64 {
        self.processes
            .values()
            .map(|p| p.session_tx)
            .sum()
    }

    /// Total cumulative traffic.
    pub fn total_session_usage(&self) -> u64 {
        self.total_session_rx() + self.total_session_tx()
    }
}
