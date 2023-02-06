use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::create("output.txt").expect("Couldn't create file!");

    file.write_all(b"Welcome to Rust!").expect("Cannot write to the file!"); //basinda b olmasi gerekiyor
}
