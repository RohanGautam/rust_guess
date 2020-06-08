use rand::Rng; // rng is a trait. that defines methods random number generators implement.
use std::cmp::Ordering;
use std::io; //the library for std io

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);
    loop {
        println!("Enter your guess");
        // `let` is used for creating variables. Variables, are immutable by default and `mut` is used
        // to explicitly make them mutable.
        // `new()` returns a new instance of a string
        // `::` -> means new is an "associated" method of string. That means its a method for the "type",
        // not a method for the *instance* of the type
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess) //&guess would be a `reference`, but those are immutable by default too. `&mut` to make that mutable too
            .expect("Failed to read line"); // expect for error handling

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,     // using match again. if matches with `Ok`, returns num
            Err(_) => continue, // if err in parsing to number, carry on
        };
        println!("You guessed {}", guess);
        //matches the input to a pattern, and executes the code after the pattern if it matches.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
