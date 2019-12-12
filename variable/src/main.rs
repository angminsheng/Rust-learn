fn main() {
    let x = 5;
    println!("The value of x is {}", x);
    let x = 6;
    println!("The value of x is {}", x);

    let mut y = 5;
    println!("The value of y is {}", y);
    y = 6;
    println!("The value of y is {}", y);

    // const is always muutable.
    const Z: u32 = 100;
    println!("The value of z is {}", Z);

    let tup: (i32, f64, u8) = (32, 64.0, 1);

    //destructuring
    let (a, b, c) = tup;
    println!("the value of first tup index is {}", a);
    println!("second index has {}", tup.1);

    //array. Array in Rust has fixed length and each element must be of the same type.

    //array are useful when we want our data to be stored in the stack rather than the heap.
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let alphabets: [u32; 5] = [1, 2, 3, 4, 5];
}
