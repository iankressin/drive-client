use serde::{Deserialize, Serialize};
use std::fs;
use std::io::prelude::*;
use std::net::{Ipv4Addr, SocketAddr, TcpStream};
use std::sync::Arc;
use std::thread;
use crate::types::Metadata;

pub struct TcpClient {
    socket_addr: SocketAddr,
    meta_list: Vec<Metadata>,
}

impl TcpClient {
    pub fn new(ip_addr: Ipv4Addr, meta_list: Vec<Metadata>) -> TcpClient {
        TcpClient {
            meta_list,
            socket_addr: format!("{}:7878", ip_addr).parse().unwrap(),
        }
    }

    pub fn conn_handler(&self) {
        let requested_files = &self.handshake().unwrap();
        &self.send_files(&requested_files);
    }

    pub fn handshake(&self) -> Result<Vec<Metadata>, std::io::Error> {
        let mut stream = &self.get_stream();
        // let mut packet = fs::read(&"./.drive/metadata.json").unwrap();
        let packet = serde_json::to_string(&self.meta_list).unwrap();
        let mut packet: Vec<u8> = packet.as_bytes().iter()
            .cloned()
            .collect();
        packet.insert(0, 0u8);

        stream.write(&packet).unwrap();

        let mut response = [0u8; 1024];
        stream.read(&mut response).unwrap();

        let eos = response.iter().position(|&r| r == 0).unwrap();
        // TODO: Sync word -> Begin and end of a packet (@lorenzolfm)
        let json = String::from_utf8_lossy(&response[..eos]);

        let incoming_metadata: Vec<Metadata> = serde_json::from_str(&json)?;

        Ok(incoming_metadata)
    }

    pub fn send_files(&self, requested_files: &Vec<Metadata>) {
        for file in requested_files {
            let mut stream = self.get_stream();
            let path = format!("./{}", &file.name_extension);

            // Created outside of the thread so requested_files 
            // does not need to have a static lifetime
            let mut packet = TcpClient::get_packet_header(&file);

            let send = thread::spawn(move || {

                match fs::read(&path) {
                    Ok(mut bytes) => {
                        packet.append(&mut bytes);
                        stream.write(&packet).unwrap();
                    },
                    Err(error) => println!("{}", error)
                }
            });

            send.join().unwrap();
        }
    }

    fn get_stream(&self) -> TcpStream {
        let socket_addr = &self.socket_addr.clone();
        TcpStream::connect(socket_addr).unwrap()
    }

    fn get_packet_header<'a>(meta: &Metadata) -> Vec<u8> {
        let mut packet = vec![0u8; 72];
        let meta_header: Vec<u8> = format!("{}:{}:{}", meta.hash, meta.name, meta.extension)
            .as_bytes()
            .iter()
            .cloned()
            .collect();

        // Offset for operation byte
        packet[1..meta_header.len() + 1].copy_from_slice(&meta_header);
        packet[0] = 1u8;

        packet
    }
}
