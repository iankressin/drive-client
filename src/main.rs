// mod tcp_client;
mod dns_builder;
mod udp_client;
mod ui;

fn main() {
    // ui::Ui::init();
    udp_client::UdpClient::query();
}
