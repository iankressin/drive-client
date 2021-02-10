use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs::{ self, File };
use std::io;
use std::io::prelude::*;
use sha1::{Sha1, Digest};


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
        println!("Configuring current directory...");

        fs::create_dir("./.drive")?;

        let mut file = fs::File::create("./.drive/metadata.json")?;
        let json = serde_json::to_string(&get_folder_metada().unwrap())?;

        file.write_all(&json.as_bytes())?;

        Ok(())
    }
}

fn hash_files(path: &str, buf: &mut [u8]) {
    let mut file = fs::File::open(&path).unwrap();
    let mut hasher = Sha1::new();
    let n = io::copy(&mut file, &mut hasher).unwrap();

    buf.copy_from_slice(&hasher.finalize())
}

// TODO: Fix known proble with filenames that contains multiple dots
// Eg: index.spec.js
fn get_file_name_and_extension(file: &String) -> (String, String) {
    // Looks for a dot at the begining or no dot at all
    let re = Regex::new(r"^\.|^[^.]*$").unwrap();

    if re.is_match(file) {
        (file.to_owned(), String::from(""))
    } else {
        let words = file.split(".").collect::<Vec<&str>>();
        let name = words[..words.len() - 1]
            .into_iter()
            .map(|i| i.to_string())
            .collect();
        let extension = words.last().unwrap().to_string();

        (name, extension)
    }
}

// TODO: Hash files
// TODO: Ugly code as hell
fn get_folder_metada() -> std::io::Result<Vec<Metadata>> {
    let mut meta = Vec::new();
    for entry in fs::read_dir("./")? {
        if let Ok(entry) = entry {
            if let Ok(metadata) = entry.metadata() {

                let (name, extension) =
                    get_file_name_and_extension(&entry.file_name().to_str().unwrap().to_string());

                let name_extension = {
                    if extension == "" {
                        name.clone()
                    } else {
                        format!("{}.{}", &name, &extension)
                    }
                };

                let mut buf = [0u8; 20];
                hash_files(&"./src/ui.rs", &mut buf);

                if !metadata.is_dir() {
                    meta.push(Metadata {
                        name_extension,
                        name,
                        extension,
                        size: metadata.len(),
                        hash: hex::encode(buf),
                    })
                }
            }
        }
    }

    Ok(meta)
}

#[test]
fn split_file_name() {
    let name_extension = get_file_name_and_extension(&"test.png".to_string());
    assert_eq!(("test".to_string(), "png".to_string()), name_extension);

    let name_extension = get_file_name_and_extension(&".gitignore".to_string());
    assert_eq!((".gitignore".to_string(), "".to_string()), name_extension);

    let name_extension = get_file_name_and_extension(&"metadata".to_string());
    assert_eq!(("metadata".to_string(), "".to_string()), name_extension);
}
