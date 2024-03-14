use std::{collections::HashMap, fs, io::Write};

const PATH_ROOT: &str = "./my_vault";
const PASS_FILE_NAME: &str = "secret";

pub struct Vault {
    name: String,
    path: String,
    encoding: String,
    passwords: HashMap<String, String>,
}

impl Vault {
    pub fn new(name: &str) -> Vault {
        Vault {
            name: name.to_string(),
            path: "./".to_string(),
            encoding: "".to_string(),
            passwords: HashMap::new(),
        }
    }

    pub fn create_vault(&self) -> std::io::Result<()> {
        let full_path = PATH_ROOT.to_string() + "/" + &self.name;
        self.create_dir(&full_path)
    }

    pub fn create_dir(&self, path: &str) -> std::io::Result<()> {
        std::fs::create_dir_all(path)
    }

    fn get_main_directory_path(&self) -> String {
        PATH_ROOT.to_string() + "/" + &self.name
    }

    fn get_main_file_path(&self) -> String {
        PATH_ROOT.to_string() + "/" + &self.name + "/" + PASS_FILE_NAME
    }
    // fn parse(&self) -> std::io::Result<()>{
    //     let buf = fs::read_to_string(self.get_main_file_path())?;
    //     for (line: String) in buf{
    //         line.split
    //         self.passwords.entry(key).unwrap_or(value);
    //     }
    //
    // }
    pub fn new_password(&self, key: &str, value: &str) -> std::io::Result<()> {
        let mut fd = std::fs::File::create(self.get_main_file_path())?;
        fd.write((key.to_string() + "=" + value).as_bytes())?;
        Ok(())
    }
}
