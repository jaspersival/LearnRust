/*Guessing game. ask the user to guess a number between 1 and a 100. If you guessed correctly,
it will say you win. If you're too high or too low it will also let you know.
*/

use rand::Rng;
use std::cmp::Ordering;

use std::io;
use std::num::ParseIntError;

fn main() {
    let mut rng = rand::thread_rng();
    let winning_num = rng.gen_range(1..=100);
    let mut number_of_tries = 0;
    let mut guessed_number;

    loop {
        guessed_number = match get_input() {
            Ok(number) => number,
            Err(err) => {
                eprintln!(
                    "Parsing Error {}, the given value should be a positive integer",
                    err
                );
                continue;
            }
        };
        if correct_number_guessed(&winning_num, &mut number_of_tries, &guessed_number) {
            break;
        }
    }
    println!("You have won with winning number {}", guessed_number);
    println!("Number of tries are: {}", number_of_tries);
}

fn get_input() -> Result<u8, ParseIntError> {
    println!("Guess a number between 0 and 100");
    let mut user_input = String::new();
    let stdin = io::stdin();
    stdin
        .read_line(&mut user_input)
        .expect("Failed to read input");
    let number = user_input.trim().parse::<u8>()?;
    Ok(number)
}
fn correct_number_guessed(
    winning_num: &u8,
    number_of_tries: &mut i32,
    &guessed_number: &u8,
) -> bool {
    *number_of_tries += 1;

    match guessed_number.cmp(winning_num) {
        Ordering::Equal => {
            return true;
        }
        Ordering::Greater => println!("Too high!. You guessed {}.", guessed_number),
        Ordering::Less => {
            println!("Too low!. You guessed {}", guessed_number);
        }
    }
    false
}
