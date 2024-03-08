fn main() {
    let mut first_name = "Marcel";
    greet(first_name);

    first_name = "Tom";
    greet(first_name);

    first_name = "Henry";
    greet(first_name);

    first_name = "Charles";
    greet(first_name);
}

// fn do_nothing() {}
// fn say_ehllo(first_name: &str) {}
// fn read_temperature() -> i32 {}
// fn sum_numbers(x: i32, y: i32) -> i32 {}

fn greet(some_name: &str) {
    println!("Hello, {}", some_name);
    println!("Welcome to the world of Rust");
    println!("Goodbye, you are going to die.");
}


