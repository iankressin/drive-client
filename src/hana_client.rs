use crate::tcp_client::TcpClient;
use hana_types::Metadata;
use crate::udp_client::UdpClient;

pub struct HanaClient;

impl HanaClient{
    pub fn send(meta: Vec<Metadata>, path: &str) {
        let server_ip = UdpClient::query().unwrap();
        let client = TcpClient::new(server_ip, meta, path);
        client.conn_handler();
    }
}
