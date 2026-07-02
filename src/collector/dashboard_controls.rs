pub fn sort_rows(
    rows: &mut Vec<(u32, String, u64, u64)>
) {
    rows.sort_by(
        |a, b| {
            (b.2 + b.3)
                .cmp(&(a.2 + a.3))
        }
    );
}

pub fn filter_idle(
    rows: Vec<(u32, String, u64, u64)>
) -> Vec<(u32, String, u64, u64)> {

    rows.into_iter()
        .filter(
            |(_, _, rx, tx)| {
                *rx > 50000 || *tx > 50000
            }
        )
        .collect()
}

pub fn filter_by_name(
    rows: Vec<(u32, String, u64, u64)>,
    search: &str,
) -> Vec<(u32, String, u64, u64)> {

    let search =
        search.to_lowercase();

    rows.into_iter()
        .filter(
            |(_, name, _, _)| {
                name
                    .to_lowercase()
                    .contains(&search)
            }
        )
        .collect()
}
