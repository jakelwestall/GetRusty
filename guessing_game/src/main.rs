// Bring in useful libraries
use std::io;
use rand::Rng;
use std::cmp::Ordering;

// Main entry point for any program. Just like in C/C++
fn main() {
	// Define a variable, and store a random number from 1 to 100 in it
	let secret_number = rand::thread_rng().gen_range(1..101);

    // Print a string on a single line of the console
    println!("Guess the number!");
    
    loop {
		println!("Please input your guess:");

		// Create a new mutable variable called guess, and assign it as an empty string
		let mut guess = String::new();

		// Grab the user input from a typed line, and handle any errors with a message
		io::stdin()
		    .read_line(&mut guess)
		    .expect("Failed to read line.");
		
		// Parse the input into an unsigned 32 bit int, so we can compare it to secret_number
		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		// Print out the user's guess
		println!("You guessed: {}", guess);
		
		// Compare guess to secret_number, and print a string based on the result ordering enum
		match guess.cmp(&secret_number) {
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {
				println!("You guessed correct!");
				break;
			}
		}
    }
}
