mod collector;

use collector::{
    collect_per_process_usage,
    discover_processes,
    discover_socket_inodes,
};

use std::collections::HashMap;

fn main() {
    println!("Starting NetScope...\n");

    let processes = discover_processes();

    let mut process_sockets = HashMap::new();

    for process in &processes {
        let sockets =
            discover_socket_inodes(process.pid);

        if !sockets.is_empty() {
            process_sockets.insert(
                process.pid,
                sockets,
            );
        }
    }

    let throughput =
        collect_per_process_usage(
            process_sockets,
        );

    println!("=== NetScope ===\n");

    let mut shown = 0;

    for process in &processes {
        if let Some((rx, tx)) =
            throughput.get(&process.pid)
        {
            println!(
                "{} → {}",
                process.pid,
                process.process_name
            );

            println!("RX: {} bytes", rx);
            println!("TX: {} bytes\n", tx);

            shown += 1;

            if shown >= 10 {
                break;
            }
        }
    }
}
