//Rust has a different approach on memory management. Memory is mangaed through a system of ownership with a set of rules that the compiler checks at compile time.

// Stack and heap differences

// Stack stores values in order it gets them and remove them in opposite order. Last in, First out.

// All data stored on stack must have known fixed size.

// Data with unknoiwn size at compile time or a size might change must be stored on the heap instead.

//Heap is less organized. When you put data on the heap, you request a certain amount of space. The operating system finds empty space that is big enough, mark it as being use and returns a pointer, which is the address of that location. The process is called allocating.

//Pushing to stack is faster as the OS never has to search for a place to store new data.

// Accessing data in a heap is slower because you have to follow a pointer to get there.

// Rules of ownership
// 1. Each value in rust has a variable that's called it's owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be  dropped.

fn main() {
    // When s comes in scope, it is valid, and it remains valid until it goees out of scope.

    let _s = "hello";
    // String literal is immutable. Hence the data type String is used. String data type is allocated on the heap and as such is able to store an amount of text that is uknown to us at compile time.

    // double colon :: is an operator that allows us to namespace this particular "from" function under the String type rather than using some name like string_from.

    // namespace are named program regions used to limit the scope of variable inside the program.
    let mut p = String::from("hello");

    p.push_str(", world");

    println!("{}", p);

    let s1 = String::from("Hello");
    let s2 = s1.clone();

    // _s cannot be modified while p can be modified. Why?

    // This is because the content of string literal is known at compile time. So the text is hardcoded directly into the final executable. They are stored in the stack.

    // With the String type, in order to support a mutable, growing piece of text, we need to allocate an amount of memory on the heap.

    //This means that:
    // Memory must be requested from the OS at runtime and we need to return the memory to the OS when we are done.

    // In Rust, the memory is automatically returned once the variavle that owns it goes out of scope.

    // When variable is out of scope, Rust calls a special function for us. This function is called drop. This is where the author of String can put the code to return the memory. Rust called drop automatically at the closing curly braces.

    // For a String type, the pointer, length and capacity are stored in the stack while the content is stored in the heap.

    // The length is how much memory the contents of the String is currently using. The capacity is the total amount of memory that the String receives from the OS.

    // When we assign a String variable to another String, we copied thhe pointer, the length and the capacity on the stack and not the data on the heap.

    // Earlier it was discussed, when a variable goes out of scope, Rust automatically calls the drop function and clean up the memory in the heap. But when both data pointers are pointing on a same location, there is a problem. When s1 and s2 goes out of scope, they will try to free the same memory. This is known as double free error and is one of the memory safety bugs we mentioned earlier. Freeing memory twice can lead to memory corruption.println!

    //Instead of trying to copy the allocated memory, Rust considred s1 to be no longer valid and therefore Rust does not need to free anything when s1 is out of scope.

    // As a result, instead of making a shallow copy like other programming languages, Rust also invalidate the first variable, and it's known as a move. We say s1 was moved to s2.

    // Ownership and functions. The semantics for passing a value to a function are similar to those for assigning a value to a variable. Passing a variable to a function will move or copy.

    let t = String::from("Hello");
    printStr(t); // s's value moves to the function
                 // println!("{}", t); // This will throws an error.
                 // s is no longer valid here.

    let x = 5;
    printNum(x); // x move to the function.
                 // But i32 is Copy, so it's okay to still use x afterwards.

    // Taking ownership and then returning ownership with every function is a bit tedious.

    // It's possible to return multiple value using a tuple

    let w1 = String::from("Hello");
    let (w2, len) = calculate_length(s1);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // returns length of String
    (s, length)
}

fn printStr(s: String) {
    println!("{}", s);
}

fn printNum(n: u32) {
    println!("{}", n);
}
