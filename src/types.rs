use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub name: String,
    pub extension: String,
    pub name_extension: String,
    pub size: u32,
    pub hash: String,
}

impl Clone for Metadata {
    fn clone(&self) -> Self {
        Metadata {
            name: self.name.clone(),
            extension: self.extension.clone(),
            name_extension: self.name_extension.clone(),
            size: self.size.clone(),
            hash: self.hash.clone(),
        }
    }
}
