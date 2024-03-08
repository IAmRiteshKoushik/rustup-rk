fn main() {
    println!("What is your name ?");
    let input = read_string();
    println!("Your name is {}", input);
}

fn read_string() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("can not read user input");
    input
}
