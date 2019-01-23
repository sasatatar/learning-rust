use std::cmp::Ordering;
use std::io;
/* The Rng trait defines methods that random number generators implement, 
and this trait must be in scope for us to use those methods */
use rand::Rng;

fn main() {

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        // parse returns a Result enum with two possible values: Ok and Err
        let guess: u32 = match guess.trim().parse() {
            // on successful parsing, return the number which will get stored in guess variable
            Ok(num) => num,
            // on error, just continue the loop
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // match takes the result of cmp method, which is an Ordering enum with 3 possible values
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
