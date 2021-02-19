// mod tcp_client;
mod dns_builder;
mod tcp_client;
mod udp_client;
mod ui;

fn main() {
    // ui::Ui::init().unwrap();
    let server_ip = udp_client::UdpClient::query().unwrap();
    let client = tcp_client::TcpClient::new(server_ip); 
    client.conn_handler();
}
