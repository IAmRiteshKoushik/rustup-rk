fn main() {
    let number = 3;
    
    // Conditions must be boolean expressions
    // unlike JavaScript, Ruby or Python which allows all kinds of expressions
    // Rust will not auto-convert non-booleans to booleans
    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    // Multiple if, elif, else
    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3, or 2");
    }

    // Alternate to ternary ops (I guess ?)
    let condition = true;
    let number = if condition { 5 } else { 6 };
    // There is a problem in the above approach, you cannot return different
    // data types in the if-else arms 
    // ILLEGAL CODE BELOW: 
    // let number = if condition { 5 } else { "six" }

    println!("The value of number is: {number}");
}
