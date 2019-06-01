
use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

#[derive(Debug)]
struct Knowledge {
    lower: u32,
    upper: u32,
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut guesses = 0;

    let mut knowledge = Knowledge {
        lower: 1,
        upper: 100,
    };

    loop {
        println!(
            "Answer is between {} and {}",
            knowledge.lower, knowledge.upper
        );
        print!("Please input your guess: ");
        io::stdout().flush().expect("Failed to flush STDOUT");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Move the cursor one up and re-write the previous line.
        // This gets rid of the newline added by stdin().read_line call :D
        print!("\u{001B}[1APlease input your guess: {}", guess.trim_end());

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!(" -> Was that really a number?");
                continue;
            }
        };
        guesses += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!(" -> Too small!");
                match knowledge.lower.cmp(&guess) {
                    Ordering::Greater => println!(
                        "You already knew the answer is at least {}, why waste your guess?!",
                        knowledge.lower
                    ),
                    _ => knowledge.lower = guess + 1,
                }
            }
            Ordering::Greater => {
                println!(" -> Too big!");
                match knowledge.upper.cmp(&guess) {
                    Ordering::Less => println!(
                        "You already knew the answer is at most {}, why waste your guess?!",
                        knowledge.upper
                    ),
                    _ => knowledge.upper = guess - 1,
                }
            }
            Ordering::Equal => {
                println!(" -> Correct! You win with {} guesses!", guesses);
                break;
            }
        }
    }
}
