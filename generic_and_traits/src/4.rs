//Struct lifetimes
use std::fmt::Display;

fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Annoucnement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
fn main() {
    let x = "Aaa";
    let y = "sdafasdf";
    let f = String::from("Buta-Gorilla is Hardik");
    longest_with_announcement(&x, &y, f);
}
