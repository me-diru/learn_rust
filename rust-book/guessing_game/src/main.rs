use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    
    println!("Guess the number!");

    let secret_num = rand::rng().random_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please provide a valid number between 1 to 100!");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Correct guess!");
                break;
            },
        }
    }

}