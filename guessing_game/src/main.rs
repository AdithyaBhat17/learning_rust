use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the guessing game");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Guess a number between 1 and 100");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // since parse () returns an enum {Ok, Err} we can use match to
        // handle errors instead of crashing the app using expect()
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // _ is a catchall, which means we catch all kinds of errors and continue
            // to the next iteration of the loop
            Err(_) => continue,
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("You guessed too low :("),
            Ordering::Greater => println!("You guessed too high :("),
            Ordering::Equal => {
                println!("Yayy you win!!!");
                break;
            }
        }
    }
}
