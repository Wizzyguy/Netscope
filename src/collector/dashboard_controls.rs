use crate::collector::ProcessSession;

/// Sort processes by total live throughput (RX/s + TX/s),
/// highest first.
pub fn sort_rows(rows: &mut Vec<ProcessSession>) {
    rows.sort_by(|a, b| {
        (b.rx_speed + b.tx_speed)
            .cmp(&(a.rx_speed + a.tx_speed))
    });
}

/// Remove completely idle processes.
///
/// A process is considered active if it is using
/// network OR CPU.
pub fn filter_idle(
    rows: Vec<ProcessSession>,
) -> Vec<ProcessSession> {
    rows.into_iter()
        .filter(|row| {
            row.rx_speed > 0
                || row.tx_speed > 0
                || row.cpu > 0.1
        })
        .collect()
}

/// Filter processes by name.
pub fn filter_by_name(
    rows: Vec<ProcessSession>,
    search: &str,
) -> Vec<ProcessSession> {
    let search = search.to_lowercase();

    rows.into_iter()
        .filter(|row| {
            row.name
                .to_lowercase()
                .contains(&search)
        })
        .collect()
}

/// Sort by process name.
pub fn sort_by_name(rows: &mut Vec<ProcessSession>) {
    rows.sort_by(|a, b| {
        a.name
            .to_lowercase()
            .cmp(&b.name.to_lowercase())
    });
}

/// Sort by PID.
pub fn sort_by_pid(rows: &mut Vec<ProcessSession>) {
    rows.sort_by(|a, b| a.pid.cmp(&b.pid));
}

/// Sort by total downloaded bytes.
pub fn sort_by_download(rows: &mut Vec<ProcessSession>) {
    rows.sort_by(|a, b| {
        b.rx_total.cmp(&a.rx_total)
    });
}

/// Sort by total uploaded bytes.
pub fn sort_by_upload(rows: &mut Vec<ProcessSession>) {
    rows.sort_by(|a, b| {
        b.tx_total.cmp(&a.tx_total)
    });
}

/// Sort by CPU usage.
pub fn sort_by_cpu(rows: &mut Vec<ProcessSession>) {
    rows.sort_by(|a, b| {
        b.cpu
            .partial_cmp(&a.cpu)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
}

/// Sort by memory usage.
pub fn sort_by_memory(rows: &mut Vec<ProcessSession>) {
    rows.sort_by(|a, b| {
        b.memory.cmp(&a.memory)
    });
}

/// Sort by cumulative session traffic.
pub fn sort_by_session(rows: &mut Vec<ProcessSession>) {
    rows.sort_by(|a, b| {
        b.total_session()
            .cmp(&a.total_session())
    });
}
