// -- MUTABILITY
// fn main() {
//     let mut x: i8 = 100;
//
//     // for i in 0..1000 {
//     //     x = x + 100;
//     // }
//     println!("x = {}", x);
// }

// -- BOOLEANS
// fn main() {
//     let is_male = true;
//     let is_above_19 = true;
//
//     if is_male {
//         println!("You are a male");
//     } else {
//         println!("You are not a male");
//     }
//
//     if is_male && is_above_19 {
//         println!("You are a legal male");
//     }
// }

// -- STRINGS
// fn main() {
// This stores a pointer to the string in the variable
// let ax = "ahrd"; // change space at runtime

// This stores the string itself in the variable
// let greeting = String::from("hello world");
// println!("{}", greeting);

// Option<char> - It can hold a character or a null value
// let char1 = greeting.chars().nth(100);
// match char1 {
//     Some(c) => println!("char1: {}", c),
//     None => println!("No character at index 1000"),
// };

// alternate way -> take a poke with runtime exceptions
// println!("char1: {}", char1.unwrap());
// }

// -- CONDITIONALS
// fn main() {
//     let is_even = true;
//
//     if is_even {
//         println!("The number is even");
//     } else if !is_even {
//         println!("The number is not even");
//     }
// }

// -- LOOPS and STRING appends
// fn main() {
//     for i in 0..10 {
//         print!("{} ", i);
//     }
//     println!("");
//
//     // Iterables -> arrays, maps, strings
//     let sentence = String::from("Hi, I am James Bond");
//     let first_word = get_first_word(sentence);
//     println!("First word is: {}", first_word);
// }
//
// fn get_first_word(sentence: String) -> String {
//     let mut ans = String::from("");
//     for char in sentence.chars() {
//         // ans.push_str(char.to_string().as_str());
//         if char == ' ' {
//             break;
//         }
//         ans.push(char);
//     }
//     return ans;
// }

// fn main() {
//     let a = 10;
//     let b = 20;
//     let sum = do_sum(a, b);
//     println!("Sum of {} and {} is {}.", a, b, sum);
// }
//
// fn do_sum(a: i32, b: i32) -> i32 {
//     return a + b;
// }

// fn main() {
//     stack_fn(); // call the function that uses the stk memory
//     heap_fn(); // call the function that uses the heap memory
//     update_string(); // call the function that changes size in heap memory
// }
//
// fn stack_fn() {
//     let a = 10;
//     let b = 20;
//     let c = a + b;
//
//     println!("Stack function: The sum of {} and {} is {}", a, b, c);
// }
//
// fn heap_fn() {
//     // Create a string which gets allocated in the heap
//     let s1 = String::from("Hello");
//     let s2 = String::from("World");
//     let combined = format!("{} {}", s1, s2);
//     println!("Heap function: Combined string is '{}'", combined);
// }
// fn update_string() {
//     // Start with a base string
//     let mut s = String::from("Initial string");
//     println!("Before update: {}", s);
//     // Pointers are printed with special directives from macros
//     println!(
//         "Capacity: {}, Length: {}, pointer: {:p}",
//         s.capacity(),
//         s.len(),
//         s.as_str()
//     );
//     // Append some text to the string
//     s.push_str(" and some additional text");
//     println!(
//         "Capacity: {}, Length: {}, pointer: {:p}",
//         s.capacity(),
//         s.len(),
//         s.as_str()
//     );
//
//     println!("After update: {}", s);
//
//     // Checking the changes in capacity, length and pointer
//     // 1. Capacity doubles when it is almost filled up
//     // 2. Length grows normally
//     // 3. Pointers change only when there is no chunk of contiguous memory
//     // which can accomodate the entire capacity
//     for _ in 0..100 {
//         s.push_str(" and some additional text");
//         println!(
//             "Capacity: {}, Length: {}, pointer: {:p}",
//             s.capacity(),
//             s.len(),
//             s.as_str()
//         );
//     }
// }

// OWNERSHIP - PART 1
// fn main() {
//     let s1 = String::from("Hi there");
//     println!("{}", s1);
//     let s2 = s1;

// Borrow of a moved value s1. Basically, the value inside of s1 has moved
// into s2 and the variable is defunct. It holds nothing and hence the
// program would not compile
//     println!("{}", s2);
// }

// -- OWNERSHIP - PART 2
// fn main() {
// This is a problem that happens only with strings and other data-types
// which require the memory allocation to by dynamic (heap). Numeric
// values have fixed sizes and everything gets stored in the stack frame
// ONLY with no involvement of the heap. Copies are made on the fly.

// For derived types -> copies take a certain overhead and thus they do
// not happen by default. You need to tell it to make the copy otherwise
// you end up passing the ownership
// let my_string = String::from("hello");
// takes_ownership(my_string.clone());

// Borrowed of moved value already
// println!("{}", my_string);
// }
//
// fn takes_ownership(some_string: String) {
//     println!("{}", some_string);
// }

// -- OWNERSHIP - PART 3
// Passing ownership back to the original variable because it is not dead
// yet until its scope is alive.
// fn main() {
//     let mut my_string = String::from("hello");
//     my_string = takes_ownership(my_string);
//     println!("{}", my_string);
// }
//
// fn takes_ownership(some_string: String) -> String {
//     println!("{}", some_string);
//     return some_string;
// }

// -- BORROWING AND REFERENCES - Part 1
// fn main() {
//     let s1: String = String::from("Hello");
//     let s2: &String = &s1;
//
//     println!("{}", s2);
//     println!("{}", s1); // This is valid
// }

// -- BORROWING AND REFRENCES - Part 2 (Immutable References)
// fn main() {
//     let my_string = String::from("Hello, Rust!");
//
//     // Immutable reference
//     borrow_variable(&my_string);
//     let s2: &String = &my_string;
//     let s3: &String = &my_string;
//     let s4: &String = &my_string;
//     let s5: &String = &my_string;
//     let s6: &String = &my_string;
//     println!("{}", my_string);
// }
//
// fn borrow_variable(some_string: &String) {
//     println!("{}", some_string);
// }

// -- BORROWING NAD REFERENCES - Part 3 (Mutable References)
// fn main() {
//     let mut s1: String = String::from("Hello");
//
//     // Have to pass it as a mutable reference
//     update_word(&mut s1);
//     println!("{}", s1);
// }
//
// fn update_word(word: &mut String) {
//     word.push_str(" World");
// }

// -- BORROWING AND REFERENCES - pART 4 (Errors with Mutable References)
// Read about Lifetimes and String-slices (&str)
// fn main() {
//     let mut s1: String = String::from("Hello");
//
//     // This does not work
//     let s2: &mut String = &mut s1;
//     let s3 = &mut s1;
//     update_word(&mut s1);
//     println!("{}", s1);
//     println!("{}", s2);
//
//     // This works
//     // update_word(&mut s1);
//     // update_word2(&mut s1);
//     // println!("{}", s1);
// }
//
// fn update_word(word: &mut String) {
//     word.push_str(" World")
// }
//
// fn update_word2(word: &mut String) {
//     word.push_str(" World2")
// }
