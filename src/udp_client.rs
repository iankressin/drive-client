use crate::dns_builder::DnsBuilder;
use bytes::Bytes;
use dns_message_parser::Dns;
use net2::UdpBuilder;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};

pub struct UdpClient;

impl UdpClient {
    pub fn query() {
        // TODO: Make it static
        let dest = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(224, 0, 0, 251)), 5353);
        let socket = UdpClient::get_upd_socket().unwrap();
        let query_packet = DnsBuilder::get_query_packet("_drive.local.");
        let mut buf = [0 as u8; 512];

        socket.connect(dest).unwrap();

        socket.send(&query_packet[..]).unwrap();

        let listener = UdpClient::get_upd_socket().unwrap();

        match listener.recv(&mut buf) {
            Ok(received) => {
                println!("received {} bytes {:?}", received, &buf[..received]);
                let packet = Bytes::copy_from_slice(&buf[..received]);
                let dns = Dns::decode(packet).unwrap();

                println!("{:?}", dns);
            }
            Err(e) => println!("recv function failed: {:?}", e),
        }

        println!("Received the packet");
    }

    fn get_upd_socket() -> Result<UdpSocket, std::io::Error> {
        Ok(UdpBuilder::new_v4()?
            .reuse_address(true)?
            .bind("0.0.0.0:8081")?)
    }
}
