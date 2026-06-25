use std::fs;

pub fn count_process_sockets(
    pid: u32,
) -> usize {
    let fd_path =
        format!(
            "/proc/{}/fd",
            pid
        );

    let entries =
        match fs::read_dir(
            fd_path
        ) {
            Ok(v) => v,

            Err(_) => {
                return 0;
            }
        };

    let mut sockets = 0;

    for entry in entries {
        if let Ok(entry) =
            entry
        {
            if let Ok(target) =
                fs::read_link(
                    entry.path()
                )
            {
                let target =
                    target
                        .to_string_lossy();

                if target
                    .starts_with(
                        "socket:"
                    )
                {
                    sockets += 1;
                }
            }
        }
    }

    sockets
}
