// All types which want to use std::fmt formatting traits require and
// implementation to be printable. Automatic implementations are only provided
// for types such as in the std library. All others must be manually implemented.

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    // Printing with {:?} is similar to {}
    // {:?} - Debug, {} - Display
    println!("{:?} months in a year.", 12);
    println!(
        "{1:?}, {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    // Structure is printable!
    println!("Now {:?} will print!", Structure(3));

    // The problem with derive is there is no control over how the
    // results look. What if I want this to show just a 7 ?
    println!("Now {:?} will print!", Deep(Structure(7)));

    // More printing but prettier
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    println!("{:#?}", peter);
}
