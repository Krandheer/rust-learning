pub mod a1;
pub mod guess_number;
pub mod variables;

fn main() {
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
    // guess_number::guess_num();
    // variables::learn_variables();
    // a1::complete_name();
    create_str();
    print!("2");
}
fn create_str() {
    let s1 = String::from("hello there");
    println!("{}", s1);
    let s2 = s1;
    println!("{}", s2);
}
