mod vault;
use vault::*;

fn main() {
    let mut v = Vault::new("test").unwrap();
    v.new_entry("lol", "asdasdasd", "wwoqkeoqwe").unwrap();
    // v.new_entry("lol", "asdasdasd", "wwoqkeoqwe").unwrap();
    // v.new_entry("xd", "asdasdasd", "wwoqkeoqwe").unwrap();
    // v.new_entry("leumaop", "asdasdasd", "wwoqkeoqwe").unwrap();
    println!("{:?}", v)
}
