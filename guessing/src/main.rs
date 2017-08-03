extern crate rand;

use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("\nGuess the number!");
    print!("Input your guess, bruh --> ");
    std::io::Write::flush(&mut std::io::stdout())
        .expect("ERROR: Failed to flush");

    let mut guess = String::new();
    std::io::stdin().read_line(&mut guess)
        .expect("ERROR: Failed to read line");
    print!("You guessed: {}", guess);

    let guess: u32 = guess.trim().parse()
        .expect("ERROR: Input must be a number");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    match guess.cmp(&secret_number) {
        Ordering::Less    => println!("Guess was too small..."),
        Ordering::Greater => println!("Guess was too big..."),
        Ordering::Equal   => println!("You guessed right!"),
    }
}
