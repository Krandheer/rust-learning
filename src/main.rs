// fn main() {
//     let vec: Vec<i32> = vec![];
//     let num = [1, 2, 3];
//     println!("{:?}", num);
//
//     if vec.is_empty() {
//         println!("true");
//     }

// println!("Guess the number!");
//
// println!("Please input your guess.");
//
// let mut guess = String::new();
//
// io::stdin()
//     .read_line(&mut guess)
//     .expect("Failed to read line");
//
// println!("You guessed: {guess}");

// let x = 4;
// println!("{}", x);
// println!("{}", x);
// }

use rand::Rng;
use std::io;

fn main() {
    println!("guess the number!");
    let secret_num = rand::thread_rng().gen_range(1..=100);
    println!("plz input your guess");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");
    println!("your guessed number is {guess}");
    println!("the secret number is: {secret_num}");
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            return;
        }
    };
    if guess == secret_num {
        println!("your guess is right");
    } else {
        println! {"your guess is wrong"}
    }
}
