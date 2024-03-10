use std::collections::*;
fn main() {
    let a = [1, 2, 3];
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    let v2 = vec![1, 2, 4];

    println!("{}", &v2[2]);
    let second = &v[0];
    match v.get(2) {
        Some(second) => println!("{}", second),
        None => println!("Index error"),
    }

    for i in &mut v {
        println!("{}", i);
    }
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{} : {} ", key, value);
    }
}
