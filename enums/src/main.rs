// enum IpAddrKind {
//     V4(u8.u8.u8.u8),
//     V5(String),
// }

// enum Message {
//     Quit,
//     Move { x: i32, y: i32},
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
// impl Message {
//     fn some_funciton() {
//         println!("Funciton");
//     }
// }
fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_value = Some(3);

    match some_value {
        Some(3) => println!("three"),
        _ => (),
    };
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        _=> None,
    }
}
