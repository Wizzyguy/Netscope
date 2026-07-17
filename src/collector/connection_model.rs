#[derive(Clone)]
pub struct ConnectionInfo {

    pub pid: u32,

    pub process_name: String,

    pub user: String,

    pub executable: String,

    pub protocol: String,

    pub local_ip: String,

    pub remote_ip: String,

    pub remote_host: String,

    pub port: u16,

    pub state: String,

}
