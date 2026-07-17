use std::fs;

#[derive(Debug, Clone, Default)]
pub struct NetworkTotals {
    pub download: u64,
    pub upload: u64,
}

fn parse_value(line: &str, index: usize) -> u64 {
    line.split_whitespace()
        .nth(index)
        .unwrap_or("0")
        .parse::<u64>()
        .unwrap_or(0)
}

pub fn read_network_totals() -> NetworkTotals {
    let mut totals = NetworkTotals::default();

    let content =
        fs::read_to_string("/proc/net/dev")
            .unwrap_or_default();

    for line in content.lines().skip(2) {
        let parts: Vec<&str> =
            line.split(':').collect();

        if parts.len() != 2 {
            continue;
        }

        let interface = parts[0].trim();

        //--------------------------------------------------
        // Ignore loopback
        //--------------------------------------------------

        if interface == "lo" {
            continue;
        }

        let stats = parts[1];

        totals.download += parse_value(stats, 0);

        totals.upload += parse_value(stats, 8);
    }

    totals
}

pub fn total_bandwidth(
    totals: &NetworkTotals,
) -> u64 {
    totals.download + totals.upload
}
