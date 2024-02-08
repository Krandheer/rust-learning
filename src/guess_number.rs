use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub(crate) fn guess_num() {
    println!("guess the number!");
    let secret_num = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("plz input your guess");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
        println!("your guessed number is {guess}");
        // println!("the secret number is: {secret_num}");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("\nusing ordering function:");
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("your number is less"),
            Ordering::Greater => println!("your number is more than secret number"),
            Ordering::Equal => {
                println!("your number is correct");
                break;
            }
        }
        println!("\nusing else if:");
        if guess == secret_num {
            println!("your guess is right");
            break;
        } else {
            println! {"your guess is wrong"}
        }
    }
}
