fn main() {
    println!("\nGuess the number!");
    print!("Input your guess, bruh --> ");
    std::io::Write::flush(&mut std::io::stdout())
        .expect("ERROR: Failed to flush");

    let mut guess = String::new();
    std::io::stdin().read_line(&mut guess)
        .expect("ERROR: Failed to read line");

    println!("You guessed: {}", guess);
}
