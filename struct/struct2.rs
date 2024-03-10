#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
//Associative function: They dont get passed self
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            height: size,
            width: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        height: 454,
        width: 2345,
    };

    let rect1 = Rectangle {
        height: 34345,
        width: 4345345,
    };

    let rect2 = Rectangle {
        height: 23,
        width: 69,
    };
    println!("rect can hold rect1 {}", rect.can_hold(&rect1));
    println!("rect can hold rect2 {}", rect.can_hold(&rect2));
    println!("rect: {:#?}", rect);

    println!("The area of rectangle is {} sq pixels", rect.area());

    let rect3 = Rectangle::square(34);

    println!("rect: {:#?}", rect3);
}
