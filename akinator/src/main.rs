use std::io::{self, Write};
use std::str::FromStr;

use rand::Rng;

fn main() {
    let number: i8 = rand::thread_rng().gen_range(1..=100);

    loop {
        print!("Try to hit my number from 1 at 100: ");
        let _ = std::io::stdout().flush();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        let attempt: Result<i8, _> = i8::from_str(input);

        match attempt {
            Ok(attempt) => match attempt.cmp(&number) {
                std::cmp::Ordering::Equal => {
                    println!("\nYou're a genius!");
                    break;
                }
                std::cmp::Ordering::Less => println!("My number is bigger... try again!\n"),
                std::cmp::Ordering::Greater => println!("My number is smaller... try again!\n"),
            },
            Err(..) => println!("Is {} a valid number?", input),
        }
    }
}
