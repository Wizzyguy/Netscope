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

// NEW
pub mod throughput;
pub use throughput::ThroughputTracker;

pub mod dashboard_controls;
pub use dashboard_controls::{filter_by_name, filter_idle, sort_rows};

pub mod keyboard;
pub use keyboard::{read_key, KeyAction};

pub mod models;
pub use models::ProcessInfo;

pub mod cpu_usage;
pub use cpu_usage::read_process_cpu;

pub mod memory_usage;
pub use memory_usage::read_process_memory;

pub mod cpu_tracker;
pub use cpu_tracker::CpuTracker;
