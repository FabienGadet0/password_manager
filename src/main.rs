mod vault;
use vault::*;

fn main() {
    let mut v = Vault::new("test").unwrap();
    println!("{:?}", v);
    //
    // v.create_vault().unwrap();
    // v.parse().unwrap();
    // v.new_password("test", "camarche").unwrap();
    // v.show_pass();
}
