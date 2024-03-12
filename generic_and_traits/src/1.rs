struct Point<T, U> {
    x: T,
    y: U,
}

impl <T, U> Point <T,U> {
    fn mixup<V, W>(self, other: Point<V,W>) -> Point<T,W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let  p = Point { x:5, y:10.69};
    let q = Point {x:"Hello", y: 'c'};

    let n = p.mixup(q);

    println!("p3,x = {}, p3.y= {}",n.x,n.y);
}