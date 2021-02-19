use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub name: String,
    pub extension: String,
    pub name_extension: String,
    pub size: u32,
    pub hash: String,
}
