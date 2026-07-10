use crate::collector::dashboard_controls::ProcessRow;

pub struct DashboardStats {
    pub total_processes: usize,
    pub active_processes: usize,
    pub total_rx: u64,
    pub total_tx: u64,
    pub search: String,
}

impl DashboardStats {
    pub fn from_rows(
        rows: &Vec<ProcessRow>,
        search: &str,
    ) -> Self {
        let total_rx = rows.iter().map(|r| r.2).sum();

        let total_tx = rows.iter().map(|r| r.3).sum();

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

fn format_bytes(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;

    let b = bytes as f64;

    if b >= GB {
        format!("{:.2} GB", b / GB)
    } else if b >= MB {
        format!("{:.2} MB", b / MB)
    } else if b >= KB {
        format!("{:.2} KB", b / KB)
    } else {
        format!("{} B", bytes)
    }
}
