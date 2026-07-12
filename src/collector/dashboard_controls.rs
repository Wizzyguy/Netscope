pub type ProcessRow = (
    u32,    // PID
    String, // Process
    u64,    // Memory (KB)
    f64,    // CPU
    u64,    // RX Total
    u64,    // TX Total
    u64,    // RX/s
    u64,    // TX/s
);

pub fn sort_rows(rows: &mut Vec<ProcessRow>) {
    rows.sort_by(|a, b| (b.4 + b.5).cmp(&(a.4 + a.5)));
}

pub fn filter_idle(rows: Vec<ProcessRow>) -> Vec<ProcessRow> {
    rows.into_iter()
        .filter(|(_, _, _, _, rx, tx, _, _)| *rx > 50_000 || *tx > 50_000)
        .collect()
}

pub fn filter_by_name(rows: Vec<ProcessRow>, search: &str) -> Vec<ProcessRow> {
    let search = search.to_lowercase();

    rows.into_iter()
        .filter(|(_, name, _, _, _, _, _, _)| name.to_lowercase().contains(&search))
        .collect()
}
