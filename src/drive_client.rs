use crate::tcp_client::TcpClient;
use hana_types::Metadata;
use crate::udp_client::UdpClient;
use serde::Deserialize;

pub struct DriveClient;

impl DriveClient {
    // Receives JSON or Bytes (Vec<u8>)?
    // pub fn send(json_meta: &String) {
    pub fn send(meta: Vec<Metadata>) {
        // let meta: Vec<Metadata> = serde_json::from_str(&json_meta).unwrap();
        let server_ip = UdpClient::query().unwrap();
        let client = TcpClient::new(server_ip, meta);
        client.conn_handler();
    }
}
