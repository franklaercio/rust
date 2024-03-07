use std::io;
use std::str::FromStr;

use rand::Rng;

fn main() {
    let number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Try to hit my number: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        let attempt: Result<i32, _> = i32::from_str(input);

        match attempt {
            Ok(attempt) => match attempt.cmp(&number) {
                std::cmp::Ordering::Equal => {
                    println!("\nYou're a genius!");
                    break;
                }
                std::cmp::Ordering::Less => println!("My number is bigger... try again!"),
                std::cmp::Ordering::Greater => println!("My number is smaller... try again!"),
            },
            Err(..) => println!("Is {} a number?", input),
        }
    }
}
