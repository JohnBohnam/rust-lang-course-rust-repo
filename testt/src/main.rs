use ju_tcs_rust_23_19::head;
use std::path::PathBuf;

fn main() {
    // println!("Hello, world!");
    let hh = head(PathBuf::from("./src/main.rs"), 10);
    for line in hh {
        println!("{}", line);
    }
}