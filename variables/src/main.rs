
fn main() {
    basic_loop();
    loop_label();
    while_loop();
    while_another();
    for_loop_first_variant();
    for_loop_second_variant();
}

fn basic_loop(){

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            
            // counter = counter * 2;
            // break;

            break counter * 2;

            // The above two approaches are different from each other
            // Approach 1 : Computes and breaks the loop after a certain
            // condition is met but the loop does not have a return type
            // Approach 2 : Tokens associated with the "break" keyword are
            // returned and stored in the `result` variable.
        }
    };
    println!("The result is {result}")
}

fn loop_label() {
    
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn while_another() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }
}

fn for_loop_first_variant() {
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!");
}

fn for_loop_second_variant() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is: {element}");
    }
}
