#![allow(dead_code)]

mod cipher;
mod db;
mod ui;
mod vault;

use vault::*;

fn main() {
    db::run();
    // ui::run().unwrap();
    // let mut v = Vault::new("test").unwrap();
    // v.new_entry("lol", "asdasdasd", "wwoqkeoqwe").unwrap();
    // v.print_passwords()
}
