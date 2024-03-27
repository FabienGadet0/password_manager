#![allow(dead_code)]

mod ui;
mod vault_db;

use vault_db::*;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let v: Vault = Vault::new("test").await.unwrap();
    println!("{:?}", v.get_entry("lol").await.unwrap());
    // v.new_entry("lol", "asdasdasd", "wwoqkeoqwe").await.unwrap();
    // v.print_passwords().await.unwrap();
    Ok(())
}
