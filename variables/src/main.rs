fn main() {
    let x = 5;
    println!("{}", x);
    let x = "Archit";
    println!("{}", x);

    const AGE: u32 = 20;
    //const cannot be mutated
    println!("{}", AGE);

    let tuple = ("archit", 1000_00);

    let (name, money) = tuple;

    let money = tuple.1;

    println!("{}", money);

    let array = [0; 8]; //length 8 with zeros
    let array = [1, 2, 69, 4];
    my_function(54,2332);

    //loops
    
    let mut ct =0;
    let result = loop{
        ct+=1;
        if  ct==10  {
            break ct;
        }
    };
    println!("The counter is {}", result);
    ct =0 ;
    while ct !=10 {
        ct+=1;
    }
    println!("The counter is {}", result);


    for ele in array.iter() {
        println!("the value is {}", ele);
    }
}

fn my_function(a:i32,b:i32) -> i32{
    println!("funciton is running {}",a+b);
    //return a+b;
    a+b
}
