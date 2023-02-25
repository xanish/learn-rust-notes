// import the io utils from standard library
// if we dont add import we can instead access stdin as
// std::io::stdin()
use std::io;

// import the Rng trait (is this trait similar to traits in PHP?)
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // start..=end (both are inclusive)
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Your guess?");

        // let to declare a variable, example let a = 10;
        // by default all rust variables are immutable
        // to make any variable mutable use the mut keyword
        // guess needs to be mutable since we need to take it as input
        let mut guess = String::new();

        // open the stdin and read a line from the user
        // line is read into the variable guess (passed by reference)
        // also references are by default immutable even if the variable
        // guess was mutable, so we need to specify that its a mutable reference
        // .read_line() will add user input to guess but it will also return
        // a Result enum, this enum has 2 states Ok and Err, it also has an expect()
        // method on it which can be called
        // if state is Err expect will crash the program and display the msg
        // string passed to it
        // if state is Ok expect will just take the value held by the enum and return
        // it (for read_line it will be number of bytes)
        // if we dont handle expect rust compile will throw a warning to make us aware
        // about the possible missed error handling
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read guess");

        // parse converts the string to the LHS side of variables number type
        // let guess: u32 = guess.trim().parse().expect("Guess must be a number!");

        // since above statement will crash if we enter a non number
        // do handle things gracefully we can handle both enums on Result
        // and on Err just continue to read another guess
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,     // pattern 1 for match
            Err(_) => continue, // pattern 2 for match, _ is similar to catchall value (which means on any error)
        };

        // can also interpolate values like
        // println!("x = {} and y = {}", x, y);
        // println!("x = {x} and y = {y}");
        println!("You guessed {guess}");

        // Ordering type is an enum and has Less, Greater and Equal variants / values
        // match feels like it works the same way as switch statements?
        // docs calls it as arms and matching the pattern on each arm and executing the correct one
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
