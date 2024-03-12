fn a () -> i32 {
   b ()
}
fn b () -> i32 {
   22
}

fn main() {
    let num =  a() ;
    if num==22 {
        panic!("Don't pass in 22!");
    } 
}
