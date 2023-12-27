use std::io;

// Inorder to obtain user input we need to bring the input/output lib into scope
// This library comes from the standard library called "std"

/* std::prelude [https://doc.rust-lang/org/std/prelude/index.html]
*
*  If we had to import every little thing that we use
*  Then we would have to import a lot of things but at
*  the same time, we would not want to import things
*  that a program does not use. Hence, a balance is made
*  with "prelude".
*
*  Things that are not in the prelude need to be brought
*  into scope explicitly using the "use" statements.
*
*/

use rand::Rng;

/*
* The Rng trait defines methods that random number generators implement, and this trait must
* be in scope for us too use those methods. "Traits" would be covered in detail later on.
*/

use std::cmp::Ordering;

/*
* The Ordering type is another enum and ahs the variants Less, Greater and Equal. These
* are three outcomes that are possible when comparing two values.
*/

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    /* NOTE:
     * The .thread_rng() function gives us the random number generator that is local
     * to the current thread of execution nad is seeded by the operating system.
     *
     * Then we call the .gen_range() method on the random number generator. This method
     * is defined by the Rng trait. The gen)range method takes a range expression as
     * an argument and generates a random number in the range.
     * Our range expression is of type (start..=end) which means it in inclusive of both.
     */

    // Printing out the auto-generated number. (for testing purposes ONLY)
    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        // Variable to store user input (be default rust variables are immutable)
        // String::new() returns a new instance of the string type
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // Store the input inside mutable guess variable (no overwriting)
            .expect("Failed to read line"); // When the operation fails, the error is handle

        /* NOTE: &mut guess
         *
         * We use &mut guess instead of &guess because references are immutable by default
         * In order to change them, the mut keyword needs to be specified.
         *
         * NOTE: .expect()
         * read_line() puts the user's entry into the string that we pass too it, but it also returns
         * a Result value. Result is an enum, which is a type that can be in one of multiple possible
         * states. Each possible state is called a "variant". Their purpose is to encode errors
         *
         * Result's variants are Ok and Err. The Ok variant indicates the operation was successful
         * and inside OK is the successfully generated value. The Err variant means that the operation
         * failed, and Err contains information about how or why the operation failed.
         *
         * Now, the Result ENUM has methods defined on it. One such method is .expect() method
         * If the instance of result is an Err value, expect will cause the program to crash and
         * display the message that you passed as an argument to .expect().
         *
         * NOTE: Unhandled Err
         * If you do not use the .expect() block then, the code would compile with a warning msg.
         */

        // CHECKPOINT 01
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        /* NOTE: Shadowing (type conversion)
         *
         * guess was a string whereas secret_number is an integer between 1 - 100. Hence, there
         * is a type mismatch which needed to be fixed before comparisons are made. Hence, we are
         * converting guess to an unsigned integer of 32 bits by first trimming it and then
         * parsing it.
         *
         * The Parse method returns a Result type with Err in case the conversion is not possible
         * Then, the exception handling occurs where we throw a "Please type a number."
         */

        /* CHECKPOINT 2
        * Better way to do the above - Instead of crashing the program; handling the error
        * We change from an expect call to a match expression ot move from crashing to
        * handling the error. As parse returns a Result type and Result is an enum that has
        * variants Ok and Err. We are using a match expression here, as we did with Ordering.

        * If parse() is able to successfully turn the string into a number, it will return an OK
        * value that contains the reusltant number. That Ok value will match the first arm's
        * pattern, and the match expression will ust return the num value that the parse produced
        * and put inside the Ok value. The number will end up inside guess!

        * If parse() is unable to to turn the string into a number, it will return an Err value
        * that contains more information about the rror. The Err value does not match the Ok(num)
        * pattern in the first match arm, but it does mathc the Err(_) pattern in the second arm.
        * The underscore, _, is a catchall value; in this example we are saying that we want
        * to match all Err values, no matter the kind of information they have. So the program
        * executed the second arm of code. (Basically ignoring all parsing errors)
        */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // The {} inside a println() statement is placeholder which acts as a string formatter
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
            }
            Ordering::Equal => {
                println!("You win!");
                break; // Adding the break line to kill the loop
            }
            Ordering::Greater => {
                println!("Too big!");
            }
        }

        // A match expressoin is used to decide what to do nxt based on whcih variant of Ordering
        // was returned from the call to cmp with the values guess and secret_number.
    }
}
