//Dangling referneces --> points at illeagle data

fn main() {
    let string1 = String::from("abcd");

    let string2 = String::from("sadfas");
    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    //gives smallest lifeline of the two
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
