use std::io::Write;

const PATH_ROOT: &str = "./my_vault";

pub struct Vault {
    pub name: String,
    pub path: String,
    pub encoding: String,
}

impl Vault {
    pub fn create_vault(&self) -> std::io::Result<()> {
        let full_path = PATH_ROOT.to_string() + "/" + &self.name;
        self.create_dir(&full_path)
    }

    pub fn create_dir(&self, path: &String) -> std::io::Result<()> {
        std::fs::create_dir_all(path)
    }

    fn get_full_path(&self, file_name: &String) -> String {
        PATH_ROOT.to_string() + "/" + &self.name + "/" + &file_name
    }

    pub fn new_password(&self, key: &String, value: &String) -> std::io::Result<()> {
        let mut fd = std::fs::File::create(self.get_full_path(key))?;
        fd.write(value.as_bytes())?;
        Ok(())
    }
}
