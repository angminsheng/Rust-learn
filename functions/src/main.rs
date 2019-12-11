fn main() {
    println!("Hello, world!");
    another_function();
    display_number(50, 199);

    // this is an expression:
    let y = 6;
    let x = five();
    println!("the value of x is {}", x);
}


// rust does not care where we defined the function. The new function can be defined at a position after it is called.
fn another_function(){
    println!("Another function.");
}

fn display_number(x: u32, y: u32){
    println!("The value of x is {} and the value of y is {}", x,y);
}

//Rust is an expression-based language.
// Difference between statement and expression:
// Statements are instructions that perform some action and do not return a value.
// Expressions evaluate to a resulting value.


// In some langauges like C and Ruby the variable assignment returns the value of the assignment.

// Expressions evaluate to somehting and make up most of the rest of the code.
// Expressions can be part of a stament.
// let y = 6 ; the 6 is an expression that evaluates to the value 6. Calling a function is an expressions.
// The block that we use to create new scope {} is an expression.
// {let x = 3
// x+1
// }
// In his case the  block evaluates to 4.

//You can return early from a function by using the return keyword but most functions return the last exporession implicitly.

// We use arow to declare the return type.

fn five() -> i32 {
    // be careful not to write 5; here as it will turn the expression to a statement and we will  get an mismatch type error as statment does not return anything.

    5
}
