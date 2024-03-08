use std::io::{self, Read};

fn main() {
    let mut x  = 5;
    println!("The value of x is: {x}");
    x = 6; // mut is required if you want to re-assign
    println!("The value of x is: {x}");

    // Constants : Naming convention
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // Shadowing (a way to make the immutable mutable)
    // But this is different from mutability becasue we need to sue the 
    // "let" key word everytime and the new value is set in stone, unless it 
    // gets shadowed by another instance of the same 
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of x is: {x}");

    // Other benefits of shadowing - reusability of variables
    let spaces = "   ";
    let spaces = spaces.len();
    println!("There we {spaces} many spaces");

    let guess: u32 = "42".parse().expect("Not a number!");
    // Rust has 4 primary scalar types - int, float, bool, char
    // Signed numbers are stored with 2's complement representation
    // Unsigned numbers are stored directly 

    // The use of isize and usize is sepcific to indexing certain 
    // collections which require the computer's architecture chosen
    // bit value. Otherwise better to stick to defaults
    
    let a = 2.0; // uses f64 by default because of double precision
    let y: f32 = 3.0; // f32
    
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 4 - 30;
    
    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    // Boolean types
    let t = true;
    let f: bool = false; // with explicit type annotation

    // character type : Unicode Scalar Value (more than ASCII)
    // these are 4 bytes in size
    let c = 'z';
    let z: char = 'z'; // explicit type annotation

    // Compound types - Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;
    // Accessing tuple elements
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // Compound Types - Array
    let arr = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March"];
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let arr_redeclared = [3; 5]; // This means [3, 3, 3, 3, 3]
                                
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
}
