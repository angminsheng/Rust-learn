fn main() {
    // The problem here using the tuple is that we have to return the String to the calling function  so we can still use the String after the function call.
    let s1 = String::from("Hello");
    let (s2, length) = calculate_length(s1);

    // This would throw an error as the s1 was moved to the the function and returned as s2.
    // println!("the length of {} is {}.", s1, length);

    let s3 = String::from("World");
    let len = calculate_length_ref(&s3);
    println!("The value of {} is {}", s3, len);

    let mut s = String::from("Hello");
    change(&mut s);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

// Here you would define and use a calculate_length functiuon that has a reference to an object as parameter instead of taking ownership.

// These ampersands are references, and they allow you to refer to some value without taking ownership of it.
fn calculate_length_ref(s: &String) -> usize {
    // here s is a reference to the String.

    s.len()
} // Here s goes out of scope, but since it does not have an ownership of what it refers to, nothing happpens.

//The scope in which variable s is valid is the same as function parameter's scope. We don't drop what the reference point to when it goes out of scope because we don't have ownership.

// We call having references as function parameters as borrowing.
// If we try to modify something we borrow, it doesn't work.

// Note the opposite of referencing is dereferencing which is accompolished by using the dereference operator *.

fn change(some_string: &mut String) {
    // this is not a return of the function.
    some_string.push_str(", world");
}

// There is a big restriction: You can only have one mutable reference to a particular piece of data in a particular scope.

// This restriction allows for mutation but in a very controleed fashion.

// The benefit of having this restrction is that Rust can prevent data races at compile time. A data race is similar to race condition and happens when:

// Two of more pointers access the same data at the same time.
// At least one of the pointers is being used to write to the data.
// There is no mechanism being used to synchronize access to the data.

// let mut s = String::from("hello");

// let r1 = &s; // no problem
// let r2 = &s; // no problem
// let r3 = &mut s; // BIG PROBLEM

// println!("{}, {}, and {}", r1, r2, r3);

// We also cannot have mutable references while we have an imutable one. User of imutable reference don't expect the values to suddenly change out from under them.

// dangling references
// Because s is created inside dangle, when the code of dangle is finished, s will be deallocated. We tried to return a reference to it, this means that the reference would be pointin to an invalid String.

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }
