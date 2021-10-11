use std::io;

fn main() {
	bad_array_access();
}

// It's possible to store multiple variable types into one variable, through a tuple
fn tuple_type() {
	// Also valid: let tup: (i32, f64, u8) = (500, 6.4, 1);
	let tup = (500, 6.4, 1);
	
	// We can break a tuple into multiple variables
	let (x, y, z) = tup;
	
	// We can also access a single variable by it's index
	let five_hundred = tup.0;
	let six_point_four = tup.1;
	let one = tup.2;
	
	println!("The value y is {}", y);
}

fn array_type() {
	let a = [1, 2, 3, 4, 5];
	// You can define a type in the array by [type, size]
	let b: [i32; 5] = [1, 2, 3, 4, 5];
	// Arrays with all the same value can be expressed by [value; size]
	let c = [3; 5];
	
	// Accessing an array index follows typical nomenclature
	let first = a[0];
    let second = a[1];
}

// The following will compile fine, but will not handle having an input
// greater than the array size, and will crash if user inputs a large number
fn bad_array_access() {
	let a = [1, 2, 3, 4, 5];
	let mut index = String::new();
	
	println!("Please enter an index.");
	
	io::stdin()
		.read_line(&mut index)
		.expect("Error gathering input.");
	
	let index: usize = index
		.trim()
		.parse()
		.expect("Index is not a number.");
	
	let element = a[index];
	
	println!("The element at index {} is {}", index, element);
}
