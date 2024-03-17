#![allow(dead_code)]

use std::io::Read;
use std::io::Write;
// use std::io::Write;
const PATH_ROOT: &str = "./my_vault";
const PASS_FILE_NAME: &str = "/secret.json";

// add new inputs
// encrypt.

// fn save_passwords(entries: &[PasswordEntry], filename: &str) -> Result<(), std::io::Error> {
//     let json_data: Value = serde_json::from_slice(&entries)?; // Serialize entries to JSON
//     let serialized_data = to_string(&json_data)?;
//     std::fs::write(filename, serialized_data)  // Write JSON data to file
// }

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct PasswordEntry {
    website: String,
    username: String,
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
    pub fn new(name: &str) -> anyhow::Result<Vault> {
        let mut v = Vault {
            name: name.to_string(),
            path: "./".to_string(),
            encoding: "".to_string(),
            passwords: Vec::new(),
        };
        v.create_files()?;
        v.passwords = v.parse()?;
        Ok(v)
    }

    fn parse(&self) -> anyhow::Result<Vec<PasswordEntry>> {
        let secret_file_path = self.get_secret_path(false);
        let mut file = std::fs::File::open(secret_file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        if contents.is_empty() || contents == "[{}]" {
            return Ok(Vec::new());
        }
        let a = serde_json::from_reader(file)?;
        Ok(a)
    }

    pub fn create_files(&self) -> anyhow::Result<()> {
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

    pub fn new_entry(&mut self, website: String, username: String, password: String) {
        self.passwords.push(PasswordEntry {
            website: website,
            username: username,
            password: password,
        });
    }
    // pub fn new_password(&mut self, key: &str, value: &str) -> Result<(), std::io::Error> {}
}
