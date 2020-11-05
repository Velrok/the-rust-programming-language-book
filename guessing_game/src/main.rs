use rand::Rng; // Imports the  gen_range trait, which we need for random numbers.
use std::cmp::Ordering; // Help with comparing things in a way that makes sure we handles all possible outcomes.
use std::io; // Read user input.

fn main() {
    println!("Guess the number!");

    // random number between 1-100
    let secret_number = rand::thread_rng().gen_range(1, 101); // inclusive on the lower bound, exclusive on the upper bound

    //println!("Secret number is : {}", secret_number);

    let mut attempts = 0;

    // Endless loop. We exit once you guess correctly.
    loop {
        attempts += 1;
        println!("Please input your guess.");
        // Grab space in memory to write the user input into.
        // Need to start with a new String every round, otherwise read_line just seams to append to
        // the existing one.
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess) // This returns a Result type with variants Ok and Err, which have to be handled.
            .expect("Failed to read line"); // .expect is defined on Result and just system exits with message on Err or passes though the value on Ok.

        // Try to parse user input as unsigned int.
        // Ignores anything it can not parse rather than exiting with error.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Use .cmp from std::cmp crait.
        // It returns an Enum (or Sum type) and forces us to handle all possible comparison
        // outcomes.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! After {} attempts.", attempts);
                break; // exit loop and also programm
            }
        }

        println!("You guessed: {}", guess);
    }
}
