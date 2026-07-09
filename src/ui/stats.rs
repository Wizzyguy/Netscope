use crate::ui::format::format_bytes;

pub struct DashboardStats {
    pub total_processes: usize,
    pub active_processes: usize,
    pub total_rx: u64,
    pub total_tx: u64,
    pub search: String,
}

impl DashboardStats {
    pub fn from_rows(
        rows: &Vec<(u32, String, u64, u64)>,
        search: &str,
    ) -> Self {
        let mut total_rx = 0;
        let mut total_tx = 0;

        for (_, _, rx, tx) in rows {
            total_rx += *rx;
            total_tx += *tx;
        }

        Self {
            total_processes: rows.len(),
            active_processes: rows.len(),
            total_rx,
            total_tx,
            search: if search.is_empty() {
                "None".to_string()
            } else {
                search.to_string()
            },
        }
    }

    pub fn rx_string(&self) -> String {
        format_bytes(self.total_rx)
    }

    pub fn tx_string(&self) -> String {
        format_bytes(self.total_tx)
    }
}
