use std::cmp::Ordering;
use std::io; // bringing the standard library into the programs scope
// standard library allows for easy comparison between two objects
use rand::Rng; // bringing in a third part library for random number generation

pub fn main() {
    println!("Starting guessing number game!");
    let secret_number = rand::thread_rng().gen_range(1..100); // initiates a generator for a range

    loop {
        // allows us to keep guessing the number until the user guesses the correct answer
        println!("Please guess your number between 1 and 100");

        let mut guess = String::new(); // initialises a mutable string variable

        io::stdin() // calls the standard input
            .read_line(&mut guess) // receives the users input and stores it to guess
            .expect("Failed to read the line"); // this handles the error handling of the input

        // takes the defined string object. Two objects with this variable name exist now.
        let guess: u32 = match guess // Match means this doesn't crash
            .trim() // removes leading and trailing whitespace
            .parse()
        {
            // parses the string into another type
            Ok(num) => num, // on the expression being okay, returns okay value with the number
            Err(_) => continue, // on error the underscore catches all errors and then restarts the loop
        };

        // This
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!"); // when the correct number is guessed line is printed and loop is broken
                break;
            }
        }

        println!("You failed to guess the number. You guessed: {guess}.")
    }
}
