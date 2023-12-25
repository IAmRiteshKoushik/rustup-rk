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

fn main() {
    println!("Guess the number");

    println!("Please input your guess.");

    // Variable to store user input (be default rust variables are immutable)
    // String::new() returns a new instance of the string type
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess) // Store the input inside mutable guess variable (no overwriting)
        .expect("Failed to read line"); // When the operation fails, the error is handled

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
     * NOTE : Unhandled Err
     * If you do not use the .expect() block then, the code would compile with a warning msg.
     */

    println!("You guessed: {guess}")
}
