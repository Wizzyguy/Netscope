mod collector;

use std::collections::{HashMap, HashSet};
use std::thread;
use std::time::Duration;

fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}

fn main() {
    println!("Starting NetScope...");

    let mut previous_memory:
        HashMap<u32, u64> =
        HashMap::new();

    let mut previous_pids:
        HashSet<u32> =
        HashSet::new();

    loop {
        clear_terminal();

        let processes =
            collector::collect();

        let mut current_pids =
            HashSet::new();

        println!("=== NetScope ===\n");

        for process in processes.iter().take(10) {
            current_pids.insert(
                process.pid,
            );

            let is_new =
                !previous_pids
                    .contains(
                        &process.pid,
                    );

            println!(
                "{} → {}",
                process.pid,
                process.process_name
            );

            if is_new {
                println!(
                    "MEM: {} KB  NEW",
                    process.memory_kb
                );
            } else {
                let old =
                    previous_memory
                        .get(
                            &process.pid,
                        )
                        .copied()
                        .unwrap_or(
                            process.memory_kb,
                        );

                let delta =
                    process.memory_kb
                        as i64
                        - old as i64;

                println!(
                    "MEM: {} KB  Δ {} KB",
                    process.memory_kb,
                    delta
                );
            }

            println!(
                "{}\n",
                process.executable_path
            );

            previous_memory.insert(
                process.pid,
                process.memory_kb,
            );
        }

        let exited: Vec<u32> =
            previous_pids
                .difference(
                    &current_pids,
                )
                .copied()
                .collect();

        if !exited.is_empty() {
            println!(
                "Exited processes:"
            );

            for pid in exited {
                println!(
                    "PID {}",
                    pid
                );
            }

            println!();
        }

        previous_pids =
            current_pids;

        thread::sleep(
            Duration::from_secs(1)
        );
    }
}
