mod collector;

use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}

fn main() {
    println!("Starting NetScope...");

    let mut previous: HashMap<u32, u64> =
        HashMap::new();

    loop {
        clear_terminal();

        let processes =
            collector::collect();

        println!("=== NetScope ===\n");

        for process in processes.iter().take(10) {
            let previous_memory =
                previous.get(&process.pid);

            let delta =
                previous_memory.map(
                    |old| {
                        process.memory_kb as i64
                            - *old as i64
                    },
                );

            println!(
                "{} → {}",
                process.pid,
                process.process_name
            );

            match delta {
                Some(change) => {
                    println!(
                        "MEM: {} KB  Δ {} KB",
                        process.memory_kb,
                        change
                    );
                }

                None => {
                    println!(
                        "MEM: {} KB  NEW",
                        process.memory_kb
                    );
                }
            }

            println!(
                "{}\n",
                process.executable_path
            );

            previous.insert(
                process.pid,
                process.memory_kb,
            );
        }

        thread::sleep(
            Duration::from_secs(1)
        );
    }
}
