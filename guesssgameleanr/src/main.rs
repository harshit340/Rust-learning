use std::io;  // for taking user input

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");
    let mut guess = String::new();  // create a String to hold user input

    io::stdin()
        .read_line(&mut guess)   // read user input
        .expect("Failed to read line");

    // Parse input string into an integer
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;  // Exit the program if input is not a valid number
        }
    };

    println!("You guessed: {}", guess);
}
