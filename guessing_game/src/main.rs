// Bring in the standard io library
use std::io;

// Main entry point for any program. Just like in C/C++
fn main() {
    // Print a string on a single line of the console
    println!("Guess the number!");
    println!("Please input your guess:");

    // Create a new mutable variable called guess, and assign it as an empty string
    let mut guess = String::new();

    // Grab the user input from a typed line, and handle any errors with a message
    io::stdin().read_line(&mut guess).expect("Failed to read line.");
    // Print out the user's guess
    println!("You guessed: {}", guess);
}