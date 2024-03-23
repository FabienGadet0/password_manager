#![allow(dead_code)]

mod cipher;
mod ui;
mod vault;

use vault::*;

fn main() {
    // ui::run().unwrap();
    let mut v = Vault::new("test").unwrap();
    v.new_entry("lol", "asdasdasd", "wwoqkeoqwe").unwrap();
    v.print_passwords()
}
