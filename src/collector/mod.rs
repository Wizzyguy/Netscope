mod models;
mod process_discovery;
mod socket_discovery;
mod tcp_discovery;

pub use process_discovery::{
    discover_processes,
};

pub use socket_discovery::{
    discover_socket_inodes,
};

pub use tcp_discovery::{
    read_tcp_table,
};

pub use models::ProcessInfo;
