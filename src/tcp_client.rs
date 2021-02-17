use std::fs;
use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::net::{Ipv4Addr, TcpStream};
use std::thread;

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    name: String,
    extension: String,
    name_extension: String,
    size: u32,
    hash: String,
}

pub struct TcpClient;

impl TcpClient {
    pub fn conn_handler(ip_addr: Ipv4Addr) {
        let requested_files = TcpClient::handshake(&ip_addr).unwrap();
        TcpClient::send_files(&requested_files);
    }
    
    pub fn handshake(ip_addr: &Ipv4Addr) -> Result<Vec<Metadata>, std::io::Error>{
        let socket_addr = format!("{}:7878", ip_addr);

        let mut stream = TcpStream::connect(&socket_addr).unwrap();
        let mut packet = fs::read(&"./.drive/metadata.json").unwrap();
        packet.insert(0, 0 as u8);

        println!("Packet: {:?}", packet);

        stream.write(&packet).unwrap();

        let mut response = [0u8; 1024];
        stream.read(&mut response).unwrap();

        let eos = response.iter().position(|&r| r == 0).unwrap();
        // TODO: Sync word -> Begin and end of a packet -> Falar com Loriz
        let json = String::from_utf8_lossy(&response[..eos]);

        let incoming_metadata: Vec<Metadata> = serde_json::from_str(&json)?;

        Ok(incoming_metadata)
    }

    pub fn send_files(requested_files: &Vec<Metadata>) {
        // Criar threads para cada arquivo (! criar uma thread pool)
        // Ler arquivo do disco
        // Enviar para o servidor
        
        for file in requested_files {
        }

    }
}


