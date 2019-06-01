
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

    let mut knowledge = Knowledge::TotallyUnknown;

    loop {
        match knowledge {
            Knowledge::TotallyUnknown => println!("Value is between 1 - 100"),
            Knowledge::LowerBoundaryKnown(lower) => println!("Answer is > {}", lower),
            Knowledge::UpperBoundaryKnown(upper) => println!("Answer is < {}", upper),
            Knowledge::BothBoundariesKnown(lower, upper) => {
                println!("{} < answer < {}", lower, upper)
            }
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
                    Knowledge::TotallyUnknown => Knowledge::LowerBoundaryKnown(guess),
                    Knowledge::LowerBoundaryKnown(lower) => match lower.cmp(&guess) {
                        Ordering::Less => Knowledge::LowerBoundaryKnown(guess),
                        _ => {
                            println!("You already knew the answer is higher than {}, why waste your guess?!", lower);
                            knowledge
                        }
                    },
                    Knowledge::UpperBoundaryKnown(upper) => {
                        Knowledge::BothBoundariesKnown(guess, upper)
                    }
                    Knowledge::BothBoundariesKnown(lower, upper) => match lower.cmp(&guess) {
                        Ordering::Less => Knowledge::BothBoundariesKnown(guess, upper),
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
                    Knowledge::TotallyUnknown => Knowledge::UpperBoundaryKnown(guess),
                    Knowledge::LowerBoundaryKnown(lower) => {
                        Knowledge::BothBoundariesKnown(lower, guess)
                    }
                    Knowledge::UpperBoundaryKnown(upper) => match upper.cmp(&guess) {
                        Ordering::Greater => Knowledge::UpperBoundaryKnown(guess),
                        _ => {
                            println!("You already knew the answer is lower than {}, why waste your guess?!", upper);
                            knowledge
                        }
                    },
                    Knowledge::BothBoundariesKnown(lower, upper) => match upper.cmp(&guess) {
                        Ordering::Greater => Knowledge::BothBoundariesKnown(lower, guess),
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
