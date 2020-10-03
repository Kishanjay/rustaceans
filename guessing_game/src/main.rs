use std::io;
use std::cmp::Ordering;
// Rng trait is a set of functions related to random
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // function of a crate dependency defined in Cargo.lock
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    // let keyword to create a new variable
    // mut keyword makes that variable mutable
    let mut guess = String::new();

    // gets the stdin function of the io module
    // would be equal to std::io::stdin
    io::stdin()
        .read_line(&mut guess)
        // expect will catch errors
        .expect("Failed to read line");

    println!("You guessed: {}", guess)

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
