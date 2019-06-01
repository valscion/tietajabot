
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut guesses = 0;
    let mut largest_guess: Option<u32> = None;
    let mut smallest_guess: Option<u32> = None;

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
                if smallest_guess.is_none() {
                    smallest_guess = Some(guess);
                } else {
                    if smallest_guess.expect("There was no prev guess") < guess {
                        smallest_guess = Some(guess);
                    } else {
                        println!("Why did you waste your guess for nothing? That's stupid.");
                    }
                }
            }
            Ordering::Greater => {
                println!("Too big!");
                if largest_guess.is_none() {
                    largest_guess = Some(guess);
                } else {
                    if largest_guess.expect("There was no prev guess") > guess {
                        largest_guess = Some(guess);
                    } else {
                        println!("Why did you waste your guess for nothing? That's stupid.");
                    }
                }
            }
            Ordering::Equal => {
                println!("You win with {} guesses!", guesses);
                break;
            }
        }

        println!(
            "You know the number is between {} - {}",
            smallest_guess.unwrap_or(1),
            largest_guess.unwrap_or(100)
        );
    }
}
