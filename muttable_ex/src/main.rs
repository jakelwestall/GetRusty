fn main() {
	// The following code will fail, as you cannot reassign an imuttable variable
	let x = 5;
	println!("The value of x is {}", x);
	x = 6;
	println!("The value of x is {}", x);
	
	// The following code will compile just fine, as y is muttable
	let mut y = 5;
	println!("The value of x is {}", y);
	y = 6;
	println!("The value of x is {}", y);
}
