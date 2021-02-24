mod dns_builder;
mod tcp_client;
mod udp_client;
mod ui;

pub mod drive_client;
pub mod types;

fn main() {
    let meta_list = vec![types::Metadata {
        name: "fuji".to_string(),
        extension: "jpg".to_string(),
        name_extension: "fuji.jpg".to_string(),
        hash: "b0e490e762234567ebc74fade854476fe692e320".to_string(),
        size: 124093,
    }, types::Metadata {
        name: "boletin_de_ocorrencia".to_string(),
        extension: "pdf".to_string(),
        name_extension: "boletin_de_ocorrencia.pdf".to_string(),
        size: 226728,
        hash: "abb9ebcb1da1377c8dd971a06f5298159c41b24f".to_string(),
    }];

    drive_client::DriveClient::send(meta_list);
}
