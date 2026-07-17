#[derive(Clone, Debug)]
pub struct ProcessSession {
    pub pid: u32,

    pub name: String,

    //--------------------------------------------------
    // System Metrics
    //--------------------------------------------------

    pub memory: u64,

    pub cpu: f32,

    //--------------------------------------------------
    // Total Usage (from process start)
    //--------------------------------------------------

    pub rx_total: u64,

    pub tx_total: u64,

    //--------------------------------------------------
    // Live Throughput
    //--------------------------------------------------

    pub rx_speed: u64,

    pub tx_speed: u64,

    //--------------------------------------------------
    // Session Totals
    //--------------------------------------------------

    pub session_rx: u64,

    pub session_tx: u64,
}

impl ProcessSession {
    pub fn total_session(&self) -> u64 {
        self.session_rx + self.session_tx
    }

    pub fn total_process(&self) -> u64 {
        self.rx_total + self.tx_total
    }
}
