use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use super::{
    TcpConnection,
    parse_tcp_line,
};

pub fn read_tcp_connections() -> Vec<TcpConnection> {

    let file = match File::open("/proc/net/tcp") {

        Ok(file) => file,

        Err(_) => return Vec::new(),

    };

    let reader = BufReader::new(file);

    let mut connections = Vec::new();

    for line in reader.lines().skip(1) {

        if let Ok(line) = line {

            if let Some(connection) =
                parse_tcp_line(&line)
            {
                connections.push(connection);
            }

        }

    }

    connections

}
