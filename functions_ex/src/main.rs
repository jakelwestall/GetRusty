fn main() {
    println!("Hello from the main function.");
    
    another_function();
    what_value(5, 6);
    
    let x = five();
    let y = plus_one(x);
    println!("Five is {}, plus one is {}", x, y);
}

fn another_function() {
	println!("Hello from another function.");
}

// Parameters must be typed
fn what_value(x: i32, y: i32) {
	println!("The value of x is {}", x);
	println!("The value of y is {}", y);
}

// Function return types are expressed via -> type
// By default the function will return the last expression in a code block
// unless return is explicitly called
fn five() -> i32 {
	5
}

// Adding a ; at the end of the implicit return will cause an error
// because that changes the expression to a statement
fn plus_one(x: i32) -> i32 {
	x + 1
}
