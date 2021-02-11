// mod tcp_client;
mod dns_builder;
mod tcp_client;
mod udp_client;
mod ui;

fn main() {
    let server_ip = udp_client::UdpClient::query().unwrap();

    tcp_client::TcpClient::send_files(server_ip);
}
