use crate::collector::dashboard_controls::ProcessRow;

pub struct DashboardStats {
    pub total_processes: usize,
    pub active_processes: usize,
    pub total_rx: u64,
    pub total_tx: u64,
    pub search: String,
}

impl DashboardStats {
    pub fn from_rows(rows: &Vec<ProcessRow>, search: &str) -> Self {
        Self {
            total_processes: rows.len(),
            active_processes: rows.len(),

            total_rx: rows.iter().map(|r| r.4).sum(),

            total_tx: rows.iter().map(|r| r.5).sum(),

            search: if search.is_empty() {
                "None".into()
            } else {
                search.to_string()
            },
        }
    }

    pub fn rx_string(&self) -> String {
        crate::ui::format::format_bytes(self.total_rx)
    }

    pub fn tx_string(&self) -> String {
        crate::ui::format::format_bytes(self.total_tx)
    }
}
