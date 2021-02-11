use crate::dns_builder::DnsBuilder;
use bytes::Bytes;
use dns_message_parser::rr::{A, RR};
use dns_message_parser::Dns;
use net2::UdpBuilder;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};

pub struct UdpClient;

impl UdpClient {
    pub fn query() -> Result<(Ipv4Addr), std::io::Error> {
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
                let server_addr = UdpClient::get_server_addr(&buf[..received]).unwrap();
                Ok((server_addr))
            }
            Err(e) => Err(e),
        }
    }

    fn get_server_addr(dns_response: &[u8]) -> Result<Ipv4Addr, ()> {
        let packet = Bytes::copy_from_slice(dns_response);
        let Dns { answers, .. } = Dns::decode(packet).unwrap();
        // let answer =
        // let RR::A(a_record) = answers.first().expect("Could not retrieve ip from dns packet");

        match answers.first().unwrap() {
            RR::A(A { ipv4_addr, .. }) => Ok(ipv4_addr.clone()),
            _ => Err(()),
        }
    }

    fn get_upd_socket() -> Result<UdpSocket, std::io::Error> {
        Ok(UdpBuilder::new_v4()?
            .reuse_address(true)?
            .bind("0.0.0.0:8081")?)
    }
}
