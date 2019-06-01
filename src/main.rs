
use rand::Rng;
use std::cmp::Ordering;
use std::io;

enum GuessDir {
    LESS,
    GREATER,
    UNKNOWN,
}
use GuessDir::*;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut guesses = 0;
    let mut previous_guess_direction = UNKNOWN;

    loop {
        println!("Please input your guess number {}.", guesses + 1);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        guesses += 1;

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                match previous_guess_direction {
                    LESS => println!("Still going smaller, eh?"),
                    GREATER => println!("Ok nice, now you know some boundaries"),
                    _ => println!("Try again :)"),
                };
                previous_guess_direction = LESS;
            }
            Ordering::Greater => {
                println!("Too big!");
                match previous_guess_direction {
                    LESS => println!("Ok nice, now you know some boundaries"),
                    GREATER => println!("Still going bigger, eh?"),
                    _ => println!("Try again :)"),
                };
                previous_guess_direction = GREATER
            }
            Ordering::Equal => {
                println!("You win with {} guesses!", guesses);
                break;
            }
        }
    }
}
