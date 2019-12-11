fn main() {
    println!("Hello, world!");
    check_number(4, 6);

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("Tha value of number is : {}", number);

    // let number = if condition { 5 } else { "six" };

    // This will not work because variable must have single type. Rust need to know at compile time which type the number variable is. Rust wouldn't be able to do that id the type of number was determined at runtime.

    for number in (1..6).rev() {
        println!("{}", number);
    }

    fahrenheit_celcius_converter('F', 200);
}

fn check_number(x: u32, y: u32) {
    if x > y {
        println!("x is bigger than y.")
    } else {
        println!("y is bigger than x.")
    }
}

// unlike Ruby and Js, Rust will not try to convert non Boolean to Boolean. You must be explicit and always provide if with a Boolean as it's condition.

// loops are useful to retry an operation you know might fail.
fn counter() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("the result of the counter is {}", result);
}

fn second_counter() {
    let mut num = 3;
    while num != 0 {
        println!("{}", num);
        num -= 1;
    }
    println!("DONE");
}

fn fahrenheit_celcius_converter(final_unit: char, input: u32) {
    let value = if final_unit == 'C' {
        (input - 32) * 5 / 9
    } else {
        // this has to be an expression not a statment. No ; !!!
        input * 9 / 5 + 32
    };
    println!("{}", value);
}
