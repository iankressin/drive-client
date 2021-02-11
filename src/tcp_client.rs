pub struct TcpClient;
use std::net::{Ipv4Addr, TcpStream};

impl TcpClient {
    pub fn send_files(ip_addr: Ipv4Addr) {
        let socket_addr = format!("{}:7878", ip_addr);

        let mut stream = TcpStream::connect(&socket_addr);

        println!("Sock addr: {}", socket_addr);
    }
}
