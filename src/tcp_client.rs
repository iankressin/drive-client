pub struct TcpClient;
use std::fs;
use std::io::prelude::*;
use std::net::{Ipv4Addr, TcpStream};

impl TcpClient {
    pub fn send_files(ip_addr: Ipv4Addr) {
        let socket_addr = format!("{}:7878", ip_addr);

        let mut stream = TcpStream::connect(&socket_addr).unwrap();

        stream.write(&fs::read(&"./.drive/metadata.json").unwrap()).unwrap();

        let mut response = [0; 128];
        stream.read(&mut response).unwrap();

        println!("Sock addr: {}", socket_addr);
    }
}
