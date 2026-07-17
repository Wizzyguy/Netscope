use crate::collector::ProcessSession;

pub struct DashboardStats {
    pub total_processes: usize,
    pub active_processes: usize,

    pub session_download: u64,
    pub session_upload: u64,

    pub cumulative_process_usage: u64,

    pub search: String,
}

impl DashboardStats {
    pub fn from_rows(
        rows: &Vec<ProcessSession>,
        search: &str,
    ) -> Self {
        let total_processes = rows.len();

        let active_processes = rows
            .iter()
            .filter(|row| {
                row.rx_speed > 0
                    || row.tx_speed > 0
                    || row.cpu > 0.1
            })
            .count();

        let session_download =
            rows.iter().map(|r| r.session_rx).sum();

        let session_upload =
            rows.iter().map(|r| r.session_tx).sum();

        let cumulative_process_usage =
            rows.iter().map(|r| r.total_process()).sum();

        Self {
            total_processes,

            active_processes,

            session_download,

            session_upload,

            cumulative_process_usage,

            search: if search.is_empty() {
                "None".into()
            } else {
                search.to_string()
            },
        }
    }

    pub fn total_session(&self) -> u64 {
        self.session_download + self.session_upload
    }
}
