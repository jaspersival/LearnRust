/*Guessing game. ask the user to guess a number between 1 and a 100. If you guessed correctly,
it will say you win. If you're too high or too low it will also let you know.
*/

use rand::Rng;
use std::cmp::Ordering;

use std::io;

fn main() {
    let mut rng = rand::thread_rng();
    let winning_num = rng.gen_range(1..=100);
    loop {
        println!("Guess a number between 0 and 100");
        let mut user_input = String::new();
        let stdin = io::stdin();
        stdin
            .read_line(&mut user_input)
            .expect("Failed to read input");
        let number = match user_input.trim().parse::<u8>() {
            Ok(number) => number,
            Err(err) => {
                eprintln!(
                    "Error {}, the given value should be a positive integer",
                    err
                );
                continue;
            }
        };

        if number > 100 {
            eprintln!("Error, the given value {} is greater than 100.", number);
            continue;
        }

        match number.cmp(&winning_num) {
            Ordering::Greater => println!("Too high!. You guessed {}.", number),
            Ordering::Less => println!("Too low!. You guessed {}", number),
            Ordering::Equal => {
                println!("You have won with winning number {}", number);
                break;
            }
        }
    }
}
