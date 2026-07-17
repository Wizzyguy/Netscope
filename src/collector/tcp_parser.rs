#[derive(Clone, Debug)]
pub struct TcpConnection {
    pub local_ip: String,
    pub local_port: u16,

    pub remote_ip: String,
    pub remote_port: u16,

    pub state: String,

    pub inode: u64,
}

fn decode_ipv4(hex: &str) -> String {
    if hex.len() != 8 {
        return "-".to_string();
    }

    let bytes = (0..4)
        .map(|i| {
            u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16)
                .unwrap_or(0)
        })
        .collect::<Vec<u8>>();

    format!(
        "{}.{}.{}.{}",
        bytes[3],
        bytes[2],
        bytes[1],
        bytes[0]
    )
}

fn decode_port(hex: &str) -> u16 {
    u16::from_str_radix(hex, 16).unwrap_or(0)
}

fn decode_state(state: &str) -> String {
    match state {
        "01" => "ESTABLISHED",
        "02" => "SYN_SENT",
        "03" => "SYN_RECV",
        "04" => "FIN_WAIT1",
        "05" => "FIN_WAIT2",
        "06" => "TIME_WAIT",
        "07" => "CLOSE",
        "08" => "CLOSE_WAIT",
        "09" => "LAST_ACK",
        "0A" => "LISTEN",
        "0B" => "CLOSING",
        _ => "UNKNOWN",
    }
    .to_string()
}

pub fn parse_address(field: &str) -> (String, u16) {
    let mut split = field.split(':');

    let ip = split.next().unwrap_or("");

    let port = split.next().unwrap_or("");

    (
        decode_ipv4(ip),
        decode_port(port),
    )
}

pub fn parse_tcp_line(line: &str) -> Option<TcpConnection> {
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.len() < 10 {
        return None;
    }

    let (local_ip, local_port) =
        parse_address(parts[1]);

    let (remote_ip, remote_port) =
        parse_address(parts[2]);

    let state = decode_state(parts[3]);

    let inode =
        parts[9].parse::<u64>().ok()?;

    Some(TcpConnection {
        local_ip,
        local_port,

        remote_ip,
        remote_port,

        state,

        inode,
    })
}
