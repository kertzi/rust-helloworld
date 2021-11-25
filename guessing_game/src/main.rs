use std::io;

fn main() {
    println!("Guessing game");

    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");

    println!("You guessed {}", guess);
}