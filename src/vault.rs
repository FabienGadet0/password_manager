#![allow(dead_code)]

use serde_json::to_vec_pretty;
use std::io::Write;
use std::io::{Seek, SeekFrom};
// use std::io::Write;
const PATH_ROOT: &str = "./my_vault";
const PASS_FILE_NAME: &str = "/secret.json";

//Todo open , parse and save json.
// add new inputs
// encrypt.

// fn save_passwords(entries: &[PasswordEntry], filename: &str) -> Result<(), std::io::Error> {
//     let json_data: Value = serde_json::from_slice(&entries)?; // Serialize entries to JSON
//     let serialized_data = to_string(&json_data)?;
//     std::fs::write(filename, serialized_data)  // Write JSON data to file
// }

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct PasswordEntry {
    // #[serde(default)]
    website: String,
    // #[serde(default)]
    username: String,
    // #[serde(default)]
    password: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Vault {
    name: String,
    path: String,
    encoding: String,
    passwords: Vec<PasswordEntry>,
}

impl Vault {
    pub fn new(name: &str) -> Result<Vault, std::io::Error> {
        let mut v = Vault {
            name: name.to_string(),
            path: "./".to_string(),
            encoding: "".to_string(),
            passwords: Vec::new(),
        };
        v.create_files()?;
        v.passwords = v.parse()?;
        // v.parse()?; v.passwords.push(PasswordEntry {
        //     website: "lol.com".into(),
        //     username: "yo".into(),
        //     password: "pass".into(),
        // });
        Ok(v)
    }

    //Todo if file is empty then return err , if file is {} then dont parse
    fn parse(&self) -> Result<Vec<PasswordEntry>, serde_json::Error> {
        let secret_file_path = self.get_secret_path(false);
        let file = std::fs::File::open(secret_file_path).expect("No file found");
        serde_json::from_reader(file)
        //Todo if the json is empty or [{}] then return an empty Vec<PasswordEntry>::new()
    }

    // Create the file, it fails if doesn't exist so we match the error and return Ok() instead
    // , if another kind of error then we propagate it.
    pub fn create_files(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(&self.get_secret_path(true))?;

        let mut fd = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(self.get_secret_path(false))?;

        // Write "{}" to the file if it's empty
        if fd.metadata()?.len() == 0 {
            writeln!(fd, "[{{}}]")?;
        }
        Ok(())
    }

    fn get_secret_path(&self, only_folder: bool) -> String {
        match only_folder {
            true => PATH_ROOT.to_string() + "/" + &self.name,
            _ => PATH_ROOT.to_string() + "/" + &self.name + PASS_FILE_NAME,
        }
    }

    // pub fn new_password(&mut self, key: &str, value: &str) -> Result<(), std::io::Error> {}
}
