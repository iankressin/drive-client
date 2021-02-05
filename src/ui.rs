use std::fs;
use serde::{ Serialize, Deserialize };
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct Metadata {
    name: String,
    extension: String,
    name_extension: String,
    size: u64,
    hash: String,
}

pub struct Ui;

impl Ui {
    pub fn init() -> std::io::Result<()> {
        println!("Configuring curret directory..."); 
                                                     
        fs::create_dir("./.drive")?;
                                                     
        let mut file = fs::File::create("./.drive/metadata.json")?;
                                                     
        let json = serde_json::to_string(&get_folder_metada().unwrap())?;
                                                     
        file.write_all(&json.as_bytes())?;           

        Ok(())
    }
}

fn get_folder_metada() -> std::io::Result<Vec<Metadata>> {
    let mut meta = Vec::new();

    for entry in fs::read_dir("./")? {
        if let Ok(entry) = entry {
            if let Ok(metadata) = entry.metadata() {
                println!("{:?}", entry.file_name().to_str().unwrap().to_string());
                meta.push(Metadata {
                    name: entry.file_name().to_str().unwrap().to_string(),
                    extension: "".to_string(),
                    name_extension: "".to_string(),
                    size: metadata.len(),
                    hash: "".to_string(),
                })
            }
        }
    }

    Ok(meta)
}
