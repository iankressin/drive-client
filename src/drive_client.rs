use crate::types::Metadata;
use crate::tcp_client::TcpClient;
use crate::udp_client::UdpClient;

pub struct DriveClient;

impl DriveClient {
    pub fn send(meta: Vec<Metadata>) {
        let server_ip = UdpClient::query().unwrap();
        let client = TcpClient::new(server_ip, meta); 
        client.conn_handler();
    }
}

