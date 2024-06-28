struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple structs (without names fields)
// This is useful when you need to reuse a tuple's data structure 
struct Color(i32, i32, i32); // RGB values
struct Point(i32, i32, i32); // x, y, z :: Coordinates

// Unit-Like Structs (without any fields)
// These are used to implement traits which will be discussed later
struct AlwaysEqual;


fn main() {
    let _subject = AlwaysEqual;
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    // --snip--
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    
    // This is a verbose way of doing it
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    // We can user struct update syntax
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,

        // Explicit initialization (prefer the next method)
        // username: username,
        // email: email,

        // Rust suggests you use shorthand declaration if your field name 
        // is same as parameter name in function
        username,
        email,
        sign_in_count: 1,
    }
}
