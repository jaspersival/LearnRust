///Develop a programme that uses a randomly generated number to select 1 of 3 (or more) functions
/// to show the user.
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let iterations = 100;

    for i in 0..iterations {
        let num = rng.gen_range(0..=10);
        match num {
            0 => println!("Oops you scored {}", num),
            5 => println!("You scored average: {}", num),
            10 => println!("Bullseye!, you score {}", num),
            _ => println!("Iteration {} has no match, scored {}", i, num),
        }
    }
}
