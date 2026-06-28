pub mod models;

pub mod process_discovery;
pub use process_discovery::discover_processes;

pub mod socket_discovery;
pub use socket_discovery::discover_socket_inodes;

pub mod tcp_discovery;
pub use tcp_discovery::read_tcp_table;

pub mod network_usage;
pub use network_usage::collect_network_usage;

pub mod per_process_throughput;
pub use per_process_throughput::collect_per_process_usage;

pub use models::ProcessInfo;
