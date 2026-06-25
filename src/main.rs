mod collector;

use std::thread;
use std::time::Duration;

fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}

fn main() {
    loop {
        clear_terminal();

        let processes =
            collector::collect();

        println!(
            "=== NetScope ===\n"
        );

        for process in processes.iter().take(10) {
            let sockets =
                collector::count_process_sockets(
                    process.pid,
                );

            println!(
                "{} → {}",
                process.pid,
                process.process_name
            );

            println!(
                "Sockets: {}",
                sockets
            );

            println!(
                "{}\n",
                process.executable_path
            );
        }

        thread::sleep(
            Duration::from_secs(1)
        );
    }
}
