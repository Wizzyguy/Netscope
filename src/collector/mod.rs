pub mod process_discovery;
pub use process_discovery::discover_processes;

pub mod socket_discovery;
pub use socket_discovery::discover_socket_inodes;

pub mod tcp_discovery;
pub use tcp_discovery::read_tcp_table;

pub mod network_usage;
pub use network_usage::{
    read_network_totals,
    total_bandwidth,
    NetworkTotals,
};

pub mod per_process_throughput;
pub use per_process_throughput::collect_per_process_usage;

pub mod dashboard_controls;
pub use dashboard_controls::{
    filter_by_name,
    filter_idle,
    sort_by_cpu,
    sort_by_download,
    sort_by_memory,
    sort_by_name,
    sort_by_pid,
    sort_by_session,
    sort_by_upload,
};

pub mod keyboard;
pub use keyboard::{
    read_key,
    KeyAction,
};

pub mod models;
pub use models::ProcessInfo;

pub mod cpu_usage;
pub use cpu_usage::read_process_cpu;

pub mod memory_usage;
pub use memory_usage::read_process_memory;

pub mod cpu_tracker;
pub use cpu_tracker::CpuTracker;

pub mod process_session;
pub use process_session::ProcessSession;

pub mod connection_model;
pub use connection_model::ConnectionInfo;

pub mod connection_info;
pub use connection_info::collect_connections;

pub mod tcp_parser;
pub use tcp_parser::{
    parse_tcp_line,
    TcpConnection,
};

pub mod tcp_table;
pub use tcp_table::read_tcp_connections;

pub mod inode_mapper;
pub use inode_mapper::build_inode_map;

pub mod process_info;
pub use process_info::{
    executable,
    process_name,
    username,
};
