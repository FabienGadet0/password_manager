mod vault;
use vault::*;

fn main() {
    let v = Vault {
        name: String::from("test"),
        path: String::from("./"),
        encoding: String::from(""),
    };
    v.create_vault().unwrap();
    v.new_password(&"gmail".to_string(), &"test".to_string());
    v.new_password(&"yo".to_string(), &"test".to_string());
    v.new_password(&"w".to_string(), &"test".to_string());
    v.new_password(&"q".to_string(), &"test".to_string());
}
