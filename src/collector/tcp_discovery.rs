use std::fs;

fn decode_ip(hex: &str) -> String {
    if hex.len() != 8 {
        return String::from("unknown");
    }

    let bytes = (0..4)
        .map(|i| {
            u8::from_str_radix(
                &hex[i * 2..i * 2 + 2],
                16,
            )
            .unwrap_or(0)
        })
        .collect::<Vec<_>>();

    format!(
        "{}.{}.{}.{}",
        bytes[3],
        bytes[2],
        bytes[1],
        bytes[0]
    )
}

fn decode_endpoint(
    raw: &str,
) -> String {
    let parts =
        raw
            .split(':')
            .collect::<Vec<_>>();

    if parts.len() != 2 {
        return raw.to_string();
    }

    let ip =
        decode_ip(parts[0]);

    let port =
        u16::from_str_radix(
            parts[1],
            16,
        )
        .unwrap_or(0);

    format!(
        "{}:{}",
        ip,
        port
    )
}

pub fn read_tcp_table()
-> Vec<Vec<String>> {
    let content =
        match fs::read_to_string(
            "/proc/net/tcp",
        ) {
            Ok(v) => v,

            Err(_) => {
                return vec![];
            }
        };

    let mut rows =
        Vec::new();

    for line in content.lines().skip(1) {
        let mut cols =
            line
                .split_whitespace()
                .map(
                    |v| v.to_string()
                )
                .collect::<Vec<_>>();

        if cols.len() > 9 {
            cols[1] =
                decode_endpoint(
                    &cols[1]
                );

            cols[2] =
                decode_endpoint(
                    &cols[2]
                );

            rows.push(cols);
        }
    }

    rows
}
