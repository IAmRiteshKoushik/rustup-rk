fn main() {
    println!("Hello, World!");

    another_function(5);
    print_labelled_measurement(5, 'm');

    let y = 6;
    // let x = (let y = 6);
    // Alternate way to handle 
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
    let x = five();
    println!("The value of x is {x}");

    let k = plus_one(21);
    println!("The value of k is {k}");
}

// Parameterized function
fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labelled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    25 // Return value 
       // We do not have a semi-colon because it is not a statement, it is 
       // an expression whose value we want to return
}

fn plus_one(x: i32) -> i32 {
    x + 1 // No semi-colon, default return type
}
