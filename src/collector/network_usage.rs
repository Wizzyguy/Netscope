use std::fs;

fn parse_value(line: &str, index: usize) -> u64 {
    line.split_whitespace()
        .nth(index)
        .unwrap_or("0")
        .parse::<u64>()
        .unwrap_or(0)
}

pub fn collect_network_usage() {
    println!("=== NETWORK ===\n");

    let content =
        fs::read_to_string("/proc/net/dev")
            .unwrap_or_default();

    let mut total_rx = 0_u64;
    let mut total_tx = 0_u64;

    for line in content.lines().skip(2) {
        let parts: Vec<&str> =
            line.split(':').collect();

        if parts.len() != 2 {
            continue;
        }

        let interface =
            parts[0].trim();

        let stats =
            parts[1];

        let rx =
            parse_value(stats, 0);

        let tx =
            parse_value(stats, 8);

        total_rx += rx;
        total_tx += tx;

        println!(
            "{}",
            interface
        );

        println!(
            "RX: {} bytes",
            rx
        );

        println!(
            "TX: {} bytes\n",
            tx
        );
    }

    println!(
        "TOTAL RX: {} bytes",
        total_rx
    );

    println!(
        "TOTAL TX: {} bytes",
        total_tx
    );
}
