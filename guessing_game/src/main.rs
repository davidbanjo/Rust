use rand::{thread_rng, Rng}; // i.e import thread_rng, Rng from rand
use std::cmp::Ordering;
use std::io;

fn main() {
    let mut rng = thread_rng();

    println!("Guess the number!");

    let secret_number = rng.gen_range(1..=10);

    // println!("The secret number is {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");
        // println!("My guess is : {secret_number}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small, Try again!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too big, Try again!"),
        }
    }
}
