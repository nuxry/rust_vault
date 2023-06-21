// Guess The Number: Normal Edition

use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn game() {
    println!("Guess the number game!");
    let secret_number: i32 = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("Enter your guess");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Enter a number");
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
        }
    }
}
