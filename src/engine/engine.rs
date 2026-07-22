use std::collections::HashMap;

use crate::collector::{
    collect_connections,
    read_process_cpu,
    ProcessInfo,
    ProcessSession,
};

use super::{
    BandwidthEngine,
    ConnectionEngine,
    CpuEngine,
    DashboardCache,
    MemoryEngine,
    SessionEngine,
};

pub struct Engine {
    //----------------------------------------------------
    // Sub Engines
    //----------------------------------------------------

    pub bandwidth: BandwidthEngine,
    pub cpu: CpuEngine,
    pub memory: MemoryEngine,
    pub session: SessionEngine,
    pub connection: ConnectionEngine,

    //----------------------------------------------------
    // Cached UI Data
    //----------------------------------------------------

    dashboard: Vec<ProcessSession>,
    cache: DashboardCache,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            bandwidth: BandwidthEngine::new(),
            cpu: CpuEngine::new(),
            memory: MemoryEngine::new(),
            session: SessionEngine::new(),
            connection: ConnectionEngine::new(),

            dashboard: Vec::new(),
            cache: DashboardCache::new(),
        }
    }

    //----------------------------------------------------
    // Build Dashboard
    //----------------------------------------------------

    pub fn update(
        &mut self,
        processes: Vec<ProcessInfo>,
        usage: HashMap<u32, (u64, u64)>,
    ) {
        self.dashboard.clear();

        //------------------------------------------------
        // Active PID list
        //------------------------------------------------

        let active: Vec<u32> =
            processes.iter().map(|p| p.pid).collect();

        self.bandwidth.cleanup(&active);
        self.cpu.cleanup(&active);
        self.memory.cleanup(&active);
        self.session.cleanup(&active);

        //------------------------------------------------
        // Build Dashboard Rows
        //------------------------------------------------

        for process in processes {
            let (rx, tx) = usage
                .get(&process.pid)
                .copied()
                .unwrap_or((0, 0));

            //------------------------------------------------
            // Update Engines
            //------------------------------------------------

            self.bandwidth.update_process(
                process.pid,
                rx,
                tx,
            );

            let cpu_ticks =
                read_process_cpu(process.pid)
                    .unwrap_or(0);

            self.cpu.update(process.pid);

            self.memory.update(process.pid);

            self.session.update(
                process.pid,
                rx,
                tx,
            );

            //------------------------------------------------
            // Read Engine Values
            //------------------------------------------------

            let (speed_rx, speed_tx) =
                self.bandwidth.process_speed(process.pid);

            let (session_rx, session_tx) =
                self.session.process(process.pid);

            let (total_rx, total_tx) =
                self.bandwidth.process_total(process.pid);

            let cpu =
                self.cpu.value(process.pid);

            let memory =
                self.memory.value(process.pid);

            //------------------------------------------------
            // Dashboard Row
            //------------------------------------------------

            self.dashboard.push(ProcessSession {
                pid: process.pid,
                name: process.process_name,

                memory,
                cpu,

                rx_total: total_rx,
                tx_total: total_tx,

                rx_speed: speed_rx,
                tx_speed: speed_tx,

                session_rx,
                session_tx,
            });
        }

        //------------------------------------------------
        // Update Dashboard Cache
        //------------------------------------------------

        self.cache.update(self.dashboard.clone());

        self.dashboard = self.cache.rows();

        //------------------------------------------------
        // Refresh Connections
        //------------------------------------------------

        self.connection.replace(
            collect_connections(),
        );
    }

    //----------------------------------------------------
    // Dashboard
    //----------------------------------------------------

    pub fn dashboard(
        &self,
    ) -> &Vec<ProcessSession> {
        &self.dashboard
    }

    //----------------------------------------------------
    // Connections
    //----------------------------------------------------

    pub fn connections(
        &self,
    ) -> &Vec<crate::collector::ConnectionInfo> {
        self.connection.connections()
    }

    //----------------------------------------------------
    // Reset
    //----------------------------------------------------

    pub fn reset(
        &mut self,
    ) {
        self.bandwidth.reset();
        self.cpu.reset();
        self.memory.reset();
        self.session.reset();

        // Uncomment this if ConnectionEngine gets a clear() method.
        // self.connection.clear();

        self.dashboard.clear();
        self.cache.clear();
    }
}
