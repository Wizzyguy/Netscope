use std::collections::HashMap;

use super::{
    build_inode_map,
    executable,
    process_name,
    read_tcp_connections,
    username,
    ConnectionInfo,
};

pub fn collect_connections() -> Vec<ConnectionInfo> {

    //----------------------------------------------------
    // Build inode -> PID map
    //----------------------------------------------------

    let inode_map = build_inode_map();

    //----------------------------------------------------
    // Read every TCP connection
    //----------------------------------------------------

    let tcp_connections = read_tcp_connections();

    //----------------------------------------------------
    // Cache process information
    //----------------------------------------------------

    let mut process_cache: HashMap<u32, (String, String, String)> =
        HashMap::new();

    let mut results = Vec::new();

    //----------------------------------------------------
    // Combine Everything
    //----------------------------------------------------

    for connection in tcp_connections {

        let pid = match inode_map.get(&connection.inode) {

            Some(pid) => *pid,

            None => continue,

        };

        let (name, exe, user) =
            process_cache
                .entry(pid)
                .or_insert_with(|| {

                    (
                        process_name(pid),
                        executable(pid),
                        username(pid),
                    )

                });

        results.push(ConnectionInfo {

            pid,

            process_name: name.clone(),

            user: user.clone(),

            executable: exe.clone(),

            protocol: protocol_name(connection.remote_port),

            local_ip: connection.local_ip,

            remote_ip: connection.remote_ip.clone(),

            remote_host: connection.remote_ip,

            port: connection.remote_port,

            state: connection.state,

        });

    }

    results
}

fn protocol_name(port: u16) -> String {

    match port {

        80 => "HTTP",

        443 => "HTTPS",

        53 => "DNS",

        22 => "SSH",

        21 => "FTP",

        25 => "SMTP",

        110 => "POP3",

        143 => "IMAP",

        3306 => "MySQL",

        5432 => "PostgreSQL",

        6379 => "Redis",

        27017 => "MongoDB",

        _ => "TCP",

    }
    .to_string()
}
