pub fn sort_rows(
    rows: &mut Vec<(u32, String, u64, u64)>
) {
    rows.sort_by(
        |a, b| {
            let a_total = a.2 + a.3;
            let b_total = b.2 + b.3;

            b_total.cmp(&a_total)
        }
    );
}

pub fn filter_idle(
    rows: Vec<(u32, String, u64, u64)>
) -> Vec<(u32, String, u64, u64)> {

    rows
        .into_iter()
        .filter(
            |r| r.2 > 0 || r.3 > 0
        )
        .collect()
}
