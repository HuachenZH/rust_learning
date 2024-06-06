use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // generate a secret number, from 1 to 100 included
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        // initialize a new emtpy string variable
        let mut guess = String::new();

        // user input 
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // if user inputs "quit", guess will be "quit\n"
        if guess.trim() == "quit" {break};

        // string to u32 (32 bit unsigned)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) { // cmp stands for compare
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
