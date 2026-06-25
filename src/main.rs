mod collector;

use std::thread;
use std::time::Duration;

fn main() {
    println!("Starting NetScope...");

    loop {
        let processes =
            collector::collect();

        println!("\n====================");
        println!(
            "Processes discovered: {}",
            processes.len()
        );

        for process in processes.iter().take(10) {
            println!(
                "{} → {} ({} KB)",
                process.pid,
                process.process_name,
                process.memory_kb
            );

            println!(
                "   {}",
                process.executable_path
            );
        }

        thread::sleep(
            Duration::from_secs(1)
        );
    }
}
