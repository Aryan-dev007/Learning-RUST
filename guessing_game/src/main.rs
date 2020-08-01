use std::io;
// To obtain user input and then print the
// result as output, we need to bring the io(input/output)
// library into scope. The io library comes from the standard library which
// is known as std.

use rand::Rng;
// "Rng" trait defines methods that random number generators implement.

use std::cmp::Ordering;
// Variants of ordering are "less, greater and equal".

fn main() {
// "fn" syntax declares a new function, the parentheses ()
// indicate there are no parameters, and curly brackets, {, starts
// body of the function.

    println!("Guess the number! ");
    // println! is a macro that prints a string to the screen.

    let secret_number = rand::thread_rng()
                            .gen_range(1, 101);
    //println!("The secret number is {} ", secret_number);

loop {
        println!("Please input your guess");

        let mut guess = String::new();
        // let statement is used to create a variable.
        // let foo = bar; --> foo is a variable
        // In RUST variables are immutable by default.
        // "mut" is used to make variable mutable.
        // let foo = bar; --> immutable
        // let mut foo = bar; --> mutable
        // On the RHS value of guess is bound to, which is result
        // of calling "String::new", a function that returns new instance of string.
        // String is a string type provided by standard library that is growable, UTF-8 encoded bit of text.
        // The "::" syntax in the "::new" line indicates that new is associated function of "String" type.
        // An associated function is implemented on a type, in this case "String".
        // "let mut guess = String::new()" line has created a mutable variable that is currently bound to new, empty
        // instance of a "String".

        io::stdin()             // Calling stdin function from the "io" module
                    // if had'nt put "use std::io" at beginning we could use "std::io::stdin".
                    // "stdin" returns instance of the "std::io::Stdin" which is a type  to represent a handle to the standard input for your terminal.


            .read_line(&mut guess)
            // "read_line (&mut, guess)" method on the standard input handle gets the input from the user, with arguments given.
            // the "&" indicates that this argument is a refernce, which gives you a way to let multiple parts of your code access
            // one piece of data without needing to copy that data into memory multiple times.
            // References are immutable by default so we need to write "&mut guess" rather than just "&guess" to make it mutable

            //
            .expect("Failed to read line");

        let guess: u32 = match guess
                        .trim() //eliminate whitespaces
                        .parse() {
                            Ok(num) => num, // the "_" is a catchall value, in this example we're saying we want to match all ERR values, no matter what info they have inside them.
                            Err(_) => continue, // parse returns a "Result" type and the "Result" is "enum" that has variants "Ok" and "Err".
                        }; // parsing to any number specifying type "u32" usigned 32 bit number

        println!("You guessed : {}",guess);       // {} Placeholoders

        match guess.cmp(&secret_number) {        //comparing guess with secret_number

            Ordering::Less => println!("To small"),
            Ordering::Greater => println!("To Big"),
            Ordering::Equal => {
                 println!("You Win!");
                 break;
            }
        }
 }
}
