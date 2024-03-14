mod vault;
use vault::*;

fn main() {
    let v = Vault::new("test");

    v.create_vault().unwrap();
    v.new_password("gmail", "test").unwrap();
    // v.new_password("s", "test").unwrap();
    // v.new_password("21", "test").unwrap();
    // v.new_password("0123", "test").unwrap();
    // v.new_password("gmail", "test").unwrap();
    // v.new_password(&"yo".to_string(), &"test".to_string());
    // v.new_password(&"w".to_string(), &"test".to_string());
    // v.new_password(&"q".to_string(), &"test".to_string());
}
