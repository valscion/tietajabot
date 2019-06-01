
use rand::Rng;
use std::cmp::Ordering;
use std::io;

#[derive(Debug)]
enum Knowledge {
    TotallyUnknown,
    LowerBoundaryKnown(u32),
    UpperBoundaryKnown(u32),
    BothBoundariesKnown(u32, u32),
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut guesses = 0;
    let mut largest_guess: Option<u32> = None;
    let mut smallest_guess: Option<u32> = None;

    let mut knowledge = Knowledge::TotallyUnknown;

    loop {
        println!("Your knowledge is {:?}", knowledge);
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
                    if largest_guess.is_some() {
                        knowledge = Knowledge::BothBoundariesKnown(
                            smallest_guess.unwrap(),
                            largest_guess.unwrap(),
                        );
                    } else {
                        knowledge = Knowledge::LowerBoundaryKnown(smallest_guess.unwrap());
                    }
                } else {
                    if smallest_guess.expect("There was no prev guess") < guess {
                        smallest_guess = Some(guess);
                        if largest_guess.is_some() {
                            knowledge = Knowledge::BothBoundariesKnown(
                                smallest_guess.unwrap(),
                                largest_guess.unwrap(),
                            );
                        } else {
                            knowledge = Knowledge::LowerBoundaryKnown(smallest_guess.unwrap());
                        }
                    } else {
                        println!("Why did you waste your guess for nothing? That's stupid.");
                    }
                }
            }
            Ordering::Greater => {
                println!("Too big!");
                if largest_guess.is_none() {
                    largest_guess = Some(guess);
                    if smallest_guess.is_some() {
                        knowledge = Knowledge::BothBoundariesKnown(
                            smallest_guess.unwrap(),
                            largest_guess.unwrap(),
                        );
                    } else {
                        knowledge = Knowledge::UpperBoundaryKnown(largest_guess.unwrap());
                    }
                } else {
                    if largest_guess.expect("There was no prev guess") > guess {
                        largest_guess = Some(guess);
                        if smallest_guess.is_some() {
                            knowledge = Knowledge::BothBoundariesKnown(
                                smallest_guess.unwrap(),
                                largest_guess.unwrap(),
                            );
                        } else {
                            knowledge = Knowledge::UpperBoundaryKnown(largest_guess.unwrap());
                        }
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
    }
}
