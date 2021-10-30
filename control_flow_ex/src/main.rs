fn main() {
    let condition = true;
    let x = if condition {5} else {6};
    // Following line won't compile, as the types don't match
    // let y = if condition {5} else {"six"};
    let y = loop_return();

    println!("Condition is {}.", basic_if_else(x));
    println!("The number is divisible by {}.", combo_if_else(x));

    counter();

    println!("The value of y is {}.", y);
}

fn basic_if_else(x: u32) -> bool {
    if x < 5 {
        true
    } else {
        false
    }
}

fn combo_if_else(x: u32) -> u32 {
    if x % 4 == 0 {
        4
    } else if x % 3 ==0 {
        3
    } else if x % 2 == 0 {
        2
    } else {
        1
    }
}

fn counter() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }

    println!("End count = {}", count);
}

fn loop_return() -> u32 {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    result
}