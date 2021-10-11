fn main() {
    println!("Hello from the main function.");
    
    another_function();
    what_value(5, 6);
}

fn another_function() {
	println!("Hello from another function.");
}

// Parameters must be typed
fn what_value(x: i32, y: i32) {
	println!("The value of x is {}", x);
	println!("The value of y is {}", y);
}
