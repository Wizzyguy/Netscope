pub type ProcessRow = (u32, String, u64, u64, u64, u64);

pub fn sort_rows(rows: &mut Vec<ProcessRow>) {
    rows.sort_by(|a, b| {
        (b.2 + b.3).cmp(&(a.2 + a.3))
    });
}

pub fn filter_idle(
    rows: Vec<ProcessRow>,
) -> Vec<ProcessRow> {

    rows.into_iter()
        .filter(|(_, _, rx, tx, _, _)| {
            *rx > 50_000 || *tx > 50_000
        })
        .collect()
}

pub fn filter_by_name(
    rows: Vec<ProcessRow>,
    search: &str,
) -> Vec<ProcessRow> {

    let search = search.to_lowercase();

    rows.into_iter()
        .filter(|(_, name, _, _, _, _)| {
            name
                .to_lowercase()
                .contains(&search)
        })
        .collect()
}
