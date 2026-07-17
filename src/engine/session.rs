use std::collections::HashMap;

#[derive(Default, Clone)]
struct SessionData {
    last_rx: u64,
    last_tx: u64,

    session_rx: u64,
    session_tx: u64,
}

pub struct SessionEngine {
    sessions: HashMap<u32, SessionData>,
}

impl SessionEngine {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }

    //----------------------------------------------------
    // Update
    //----------------------------------------------------

    pub fn update(
        &mut self,
        pid: u32,
        current_rx: u64,
        current_tx: u64,
    ) {
        let entry = self
            .sessions
            .entry(pid)
            .or_default();

        let delta_rx =
            current_rx.saturating_sub(entry.last_rx);

        let delta_tx =
            current_tx.saturating_sub(entry.last_tx);

        entry.session_rx += delta_rx;
        entry.session_tx += delta_tx;

        entry.last_rx = current_rx;
        entry.last_tx = current_tx;
    }

    //----------------------------------------------------
    // Per Process
    //----------------------------------------------------

    pub fn process(
        &self,
        pid: u32,
    ) -> (u64, u64) {
        self.sessions
            .get(&pid)
            .map(|s| {
                (
                    s.session_rx,
                    s.session_tx,
                )
            })
            .unwrap_or((0, 0))
    }

    //----------------------------------------------------
    // Totals
    //----------------------------------------------------

    pub fn total_download(
        &self,
    ) -> u64 {
        self.sessions
            .values()
            .map(|s| s.session_rx)
            .sum()
    }

    pub fn total_upload(
        &self,
    ) -> u64 {
        self.sessions
            .values()
            .map(|s| s.session_tx)
            .sum()
    }

    pub fn total(
        &self,
    ) -> u64 {
        self.total_download()
            + self.total_upload()
    }

    //----------------------------------------------------
    // Cleanup
    //----------------------------------------------------

    pub fn cleanup(
        &mut self,
        active: &[u32],
    ) {
        self.sessions
            .retain(|pid, _| active.contains(pid));
    }

    //----------------------------------------------------
    // Reset
    //----------------------------------------------------

    pub fn reset(
        &mut self,
    ) {
        self.sessions.clear();
    }
}
