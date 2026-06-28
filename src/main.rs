mod collector;

use std::{
    collections::HashMap,
    io::{stdout, Write},
    thread,
    time::Duration,
};

use collector::{
    collect_per_process_usage,
    discover_processes,
    discover_socket_inodes,
};

fn main() {
    println!("Starting NetScope...");

    loop {
        // Clear screen
        print!("\x1B[2J\x1B[1;1H");
        stdout().flush().unwrap();

        println!("=== NetScope ===\n");

        let processes = discover_processes();

        let mut process_sockets:
            HashMap<u32, Vec<String>> =
            HashMap::new();

        let mut process_names:
            HashMap<u32, String> =
            HashMap::new();

        for process in processes {
            process_names.insert(
                process.pid,
                process.process_name.clone(),
            );

            let sockets =
                discover_socket_inodes(
                    process.pid,
                );

            process_sockets.insert(
                process.pid,
                sockets,
            );
        }

        let usage =
            collect_per_process_usage(
                process_sockets,
            );

        for (pid, (rx, tx)) in usage {
            if rx == 0 && tx == 0 {
                continue;
            }

            let name =
                process_names
                    .get(&pid)
                    .map(|s| s.as_str())
                    .unwrap_or("unknown");

            println!(
                "{} → {}",
                pid,
                name
            );

            println!(
                "RX: {} bytes",
                rx
            );

            println!(
                "TX: {} bytes",
                tx
            );

            println!();
        }

        thread::sleep(
            Duration::from_secs(2),
        );
    }
}
