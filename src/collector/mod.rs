mod models;
mod process_discovery;
mod socket_discovery;

pub use models::ProcessInfo;
pub use socket_discovery::discover_socket_inodes;

pub fn collect() -> Vec<ProcessInfo> {
    let mut processes =
        process_discovery::discover_processes();

    processes.sort_by(
        |a, b| {
            b.memory_kb
                .cmp(
                    &a.memory_kb
                )
        },
    );

    processes
}
