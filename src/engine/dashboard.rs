use crate::{
    collector::ProcessSession,
    engine::Engine,
};

pub fn build_dashboard(
    engine: &Engine,
    active_processes: &[(u32, String)],
) -> Vec<ProcessSession> {
    let mut rows = Vec::new();

    for (pid, name) in active_processes {
        let (rx_total, tx_total) =
            engine.bandwidth.process_total(*pid);

        let (rx_speed, tx_speed) =
            engine.bandwidth.process_speed(*pid);

        let (session_rx, session_tx) =
            engine.session.process(*pid);

        let cpu =
            engine.cpu.value(*pid);

        let memory =
            engine.memory.value(*pid);

        rows.push(ProcessSession {
            pid: *pid,

            name: name.clone(),

            memory,

            cpu,

            rx_total,
            tx_total,

            rx_speed,
            tx_speed,

            session_rx,
            session_tx,
        });
    }

    rows
}
