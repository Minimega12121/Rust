use std::fs::{self, File};
use std::io::ErrorKind;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    // let mut f = File::open("Hello.txt")?//similar to unwrap and expect
    // f.read_to_string(&mut s)?;
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {}
