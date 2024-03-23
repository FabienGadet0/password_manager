#![allow(dead_code)]

use serde_json::from_str;
use std::collections::HashMap;
use std::io::Read;
use std::io::Write;

const PATH_ROOT: &str = "./my_vaults";
const PASS_FILE_NAME: &str = "/secret.json";

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PasswordEntry {
    website: String,
    username: String,
    password: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Vault {
    name: String,
    path: String,
    encoding: String,
    passwords: HashMap<String, PasswordEntry>,
}

impl Vault {
    pub fn new(name: &str) -> anyhow::Result<Vault> {
        let mut v = Vault {
            name: name.to_string(),
            path: "./".to_string(),
            encoding: "".to_string(),
            passwords: HashMap::new(),
        };
        v.create_files()?;
        v.parse()?;
        Ok(v)
    }

    pub fn print_passwords(&self) {
        println!("{:?}", self.passwords)
    }

    fn dump(&self) -> Result<(), anyhow::Error> {
        let file = std::fs::File::create(&self.get_secret_path(false))?;
        let mut writer = std::io::BufWriter::new(file);
        serde_json::to_writer(&mut writer, &self.passwords)?;
        writer.flush()?;
        Ok(())
    }

    pub fn parse(&mut self) -> anyhow::Result<()> {
        let secret_file_path = self.get_secret_path(false);

        let mut file = std::fs::File::open(secret_file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        if contents.is_empty() || contents.len() <= 5 {
            return Ok(());
        }

        let json_value: serde_json::Value = from_str(&contents)?;

        for (key, value) in json_value.as_object().unwrap().iter() {
            let new_entry = PasswordEntry {
                website: value["website"].as_str().unwrap().to_owned(),
                username: value["website"].as_str().unwrap().to_owned(),
                password: value["password"].as_str().unwrap().to_owned(),
            };
            self.passwords.entry(key.clone()).or_insert(new_entry);
        }

        Ok(())
    }

    pub fn new_entry(
        &mut self,
        website: &str,
        username: &str,
        password: &str,
    ) -> anyhow::Result<()> {
        self.passwords
            .entry(website.to_string())
            .or_insert(PasswordEntry {
                website: website.to_string(),
                username: username.to_string(),
                password: password.to_string(),
            });

        self.dump()
    }

    pub fn get_entry(&self, website: &str) -> std::option::Option<&PasswordEntry> {
        self.passwords.get(website)
    }

    // Create directories , create the secret file and write [{}] if it's empty to initialize it.
    pub fn create_files(&self) -> anyhow::Result<()> {
        std::fs::create_dir_all(&self.get_secret_path(true))?;

        let mut fd = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(self.get_secret_path(false))?;

        // Write "{}" to the file if it's empty
        if fd.metadata()?.len() == 0 {
            writeln!(fd, "{{}}")?;
        }
        Ok(())
    }

    fn get_secret_path(&self, only_folder: bool) -> String {
        match only_folder {
            true => PATH_ROOT.to_string() + "/" + &self.name,
            _ => PATH_ROOT.to_string() + "/" + &self.name + PASS_FILE_NAME,
        }
    }
}
