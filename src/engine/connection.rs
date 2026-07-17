use crate::collector::ConnectionInfo;

pub struct ConnectionEngine {
    connections: Vec<ConnectionInfo>,
}

impl ConnectionEngine {
    pub fn new() -> Self {
        Self {
            connections: Vec::new(),
        }
    }

    //--------------------------------------------------------
    // Replace connection snapshot
    //--------------------------------------------------------

    pub fn replace(
        &mut self,
        connections: Vec<ConnectionInfo>,
    ) {
        self.connections = connections;
    }

    //--------------------------------------------------------
    // Read-only Access
    //--------------------------------------------------------

    pub fn connections(
        &self,
    ) -> &Vec<ConnectionInfo> {
        &self.connections
    }

    //--------------------------------------------------------
    // Statistics
    //--------------------------------------------------------

    pub fn total_connections(&self) -> usize {
        self.connections.len()
    }

    pub fn unique_processes(&self) -> usize {
        use std::collections::HashSet;

        let mut set = HashSet::new();

        for connection in &self.connections {
            set.insert(connection.pid);
        }

        set.len()
    }

    //--------------------------------------------------------
    // Reset
    //--------------------------------------------------------

    pub fn clear(&mut self) {
        self.connections.clear();
    }
}
