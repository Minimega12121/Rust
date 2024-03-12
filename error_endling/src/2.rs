use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // // Result enum
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    let f = File::open("hello.txt").expect("Failed to open helklo.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opeing the file"),
    // };
}
