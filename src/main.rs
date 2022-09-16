use std::io;
use std::num::ParseIntError;
use rand::seq::SliceRandom;

fn main() {
    println!("rock paper scissors");

    loop {

        // this line will accept user input and set it as a variable named 'choice'
        let mut choice = String::new();

        // this block will read the line and issue an error message if it can't read the line
        io::stdin()
            .read_line(&mut choice)
            .expect("failed to read line");

        // this will check if the input is a number and issue an integer error
        let choice:Result<i128, ParseIntError> = choice.trim().parse();

        // if the integer error is issued then 'choice' will not be ok and it will print the if statement
        // if it is okay then it will go ahead with the else statement
        if choice.is_ok() {
            println!("that's not an option");
        } else { // this whole if else sets up a vector of a few different options and then the randomly chooses one of them and prints it out
            let answers = vec!["rock", "paper", "scissors"];
            // after the vector this chooses a random part and outputs it
            let mut rng = rand::thread_rng();
            let choice = answers.choose(&mut rng).unwrap();
            println!("{choice}");
        }
    }
}
