use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is {}", secret_number);
    loop {
        println!("Input your guess:");

        let mut guess = String::new(); //new for allocating memeory
                                       //In rust variables are immmutable by default
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big".red()),
        }
    }
}
