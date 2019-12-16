fn main() {
    let mut user1 = User {
        username: String::from("min"),
        email: String::from("min@hotmail.com"),
        sign_in_count: 12,
        active: true,
    };

    // The .. syntax specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.
    let mut user2 = User {
        username: String::from("mini"),
        email: String::from("mini@hotmail.com"),
        ..user1
    };
    user1.username = String::from("sheng");

    // We can also create structs that looks similar to tuple called tuple structs.

    // Tuple  struct have the added meaning the struct name provides but don't have names associated with their field.

    //Tuple structs is useful when you want to give the whole tuple a name and make the tuple different from other tuples, and naming each fields as in regular would be redundant.

    struct Color(i32, i32 ,i32);
    struct Point(i32 , i32 ,i32);

    let black = Color(0,0,0);
    let origin = Point(0,0,0);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
};

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 1,
        active: true,
    }
}
