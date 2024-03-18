mod vault;
use vault::*;

fn main() {
    let mut v = Vault::new("test").unwrap();
    v.new_entry("lol".into(), "asdasdasd".into(), "wwoqkeoqwe".into());
    println!("{:?}", v);
    //
    // v.create_vault().unwrap();
    // v.parse().unwrap();
    // v.new_password("test", "camarche").unwrap();
    // v.show_pass();
}
