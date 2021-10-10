// Constants are always imuttable, and cannot be used with mut
// They must be typed, and contain a value
const MAX_POINTS: u32 = 100_000;

fn main() {

}

fn bad_code() {
	// The following code will fail, as you cannot reassign an imuttable variable (commented the offending line so it will compile)
	let x = 5;
	println!("The value of x is {}", x);
	// x = 6;
	println!("The value of x is {}", x);
}

fn good_code() {
	// The following code will compile just fine, as y is muttable
	let mut y = 5;
	println!("The value of y is {}", y);
	y = 6;
	println!("The value of y is {}", y);
}

fn shaddowing_variable() {
	// Shaddowing is redefining a variable with an already defined name
	let x = 5;
	let x = x + 1;
	let x = x * 2;
	
	// Shaddowing lets us change the variable type as well
	let spaces = "     ";
	let spaces = spaces.len();
	
	// Muttables can't do this, and will fail to compile
	// let mut spaces = "     ";
	// spaces = spaces.len();
	
	println!("The value of x is {}", x);
}
