
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
use Knowledge::{BothBoundariesKnown, LowerBoundaryKnown, TotallyUnknown, UpperBoundaryKnown};

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut guesses = 0;

    let mut knowledge = TotallyUnknown;

    loop {
        match knowledge {
            TotallyUnknown => println!("Value is between 1 - 100"),
            LowerBoundaryKnown(lower) => println!("Answer is > {}", lower),
            UpperBoundaryKnown(upper) => println!("Answer is < {}", upper),
            BothBoundariesKnown(lower, upper) => println!("{} < answer < {}", lower, upper),
        }
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
                knowledge = match knowledge {
                    TotallyUnknown => LowerBoundaryKnown(guess),
                    LowerBoundaryKnown(lower) => match lower.cmp(&guess) {
                        Ordering::Less => LowerBoundaryKnown(guess),
                        _ => {
                            println!("You already knew the answer is higher than {}, why waste your guess?!", lower);
                            knowledge
                        }
                    },
                    UpperBoundaryKnown(upper) => BothBoundariesKnown(guess, upper),
                    BothBoundariesKnown(lower, upper) => match lower.cmp(&guess) {
                        Ordering::Less => BothBoundariesKnown(guess, upper),
                        _ => {
                            println!("You already knew the answer is higher than {}, why waste your guess?!", lower);
                            knowledge
                        }
                    },
                }
            }
            Ordering::Greater => {
                println!("Too big!");
                knowledge = match knowledge {
                    TotallyUnknown => UpperBoundaryKnown(guess),
                    LowerBoundaryKnown(lower) => BothBoundariesKnown(lower, guess),
                    UpperBoundaryKnown(upper) => match upper.cmp(&guess) {
                        Ordering::Greater => UpperBoundaryKnown(guess),
                        _ => {
                            println!("You already knew the answer is lower than {}, why waste your guess?!", upper);
                            knowledge
                        }
                    },
                    BothBoundariesKnown(lower, upper) => match upper.cmp(&guess) {
                        Ordering::Greater => BothBoundariesKnown(lower, guess),
                        _ => {
                            println!("You already knew the answer is lower than {}, why waste your guess?!", upper);
                            knowledge
                        }
                    },
                }
            }
            Ordering::Equal => {
                println!("You win with {} guesses!", guesses);
                break;
            }
        }
    }
}
