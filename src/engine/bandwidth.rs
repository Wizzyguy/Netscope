use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct ProcessBandwidth {
    //--------------------------------------------------------
    // Lifetime Counters
    //--------------------------------------------------------

    pub current_rx: u64,
    pub current_tx: u64,

    //--------------------------------------------------------
    // Previous Sample
    //--------------------------------------------------------

    pub previous_rx: u64,
    pub previous_tx: u64,

    //--------------------------------------------------------
    // Live Speed
    //--------------------------------------------------------

    pub speed_rx: u64,
    pub speed_tx: u64,
}

pub struct BandwidthEngine {
    processes: HashMap<u32, ProcessBandwidth>,

    total_download: u64,
    total_upload: u64,
}

impl BandwidthEngine {
    pub fn new() -> Self {
        Self {
            processes: HashMap::new(),

            total_download: 0,
            total_upload: 0,
        }
    }

    //--------------------------------------------------------
    // Update Process
    //--------------------------------------------------------

    pub fn update_process(
        &mut self,
        pid: u32,
        rx: u64,
        tx: u64,
    ) {
        let entry = self
            .processes
            .entry(pid)
            .or_default();

        //----------------------------------------------------
        // Live Speed
        //----------------------------------------------------

        entry.speed_rx =
            rx.saturating_sub(entry.previous_rx);

        entry.speed_tx =
            tx.saturating_sub(entry.previous_tx);

        //----------------------------------------------------
        // Lifetime Counters
        //----------------------------------------------------

        entry.current_rx = rx;
        entry.current_tx = tx;

        entry.previous_rx = rx;
        entry.previous_tx = tx;

        //----------------------------------------------------
        // Global Totals
        //----------------------------------------------------

        self.total_download += entry.speed_rx;
        self.total_upload += entry.speed_tx;
    }

    //--------------------------------------------------------
    // Process Queries
    //--------------------------------------------------------

    pub fn process_speed(
        &self,
        pid: u32,
    ) -> (u64, u64) {
        self.processes
            .get(&pid)
            .map(|p| {
                (
                    p.speed_rx,
                    p.speed_tx,
                )
            })
            .unwrap_or((0, 0))
    }

    pub fn process_total(
        &self,
        pid: u32,
    ) -> (u64, u64) {
        self.processes
            .get(&pid)
            .map(|p| {
                (
                    p.current_rx,
                    p.current_tx,
                )
            })
            .unwrap_or((0, 0))
    }

    //--------------------------------------------------------
    // Global Queries
    //--------------------------------------------------------

    pub fn total_download(&self) -> u64 {
        self.total_download
    }

    pub fn total_upload(&self) -> u64 {
        self.total_upload
    }

    pub fn total_bandwidth(&self) -> u64 {
        self.total_download + self.total_upload
    }

    //--------------------------------------------------------
    // Cleanup
    //--------------------------------------------------------

    pub fn cleanup(
        &mut self,
        active: &[u32],
    ) {
        self.processes
            .retain(|pid, _| active.contains(pid));
    }

    //--------------------------------------------------------
    // Reset
    //--------------------------------------------------------

    pub fn reset(&mut self) {
        self.processes.clear();

        self.total_download = 0;
        self.total_upload = 0;
    }
}
