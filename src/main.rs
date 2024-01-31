mod a1;

// simple finding sum function
fn sum(a: i32, b: i32) -> i32 {
    return a + b;
}

// simple flow control function
// fn c_flow() {
//     let age = 21;
//     if age >= 24 {
//         println!("you are not allowed inside");
//     } else {
//         println!("you are allowed inside");
//     }
// }

// practicing the match key work
fn f_match() {
    let name = "bob";
    match name {
        "hare" => println!("hello mister was waitng for you"),
        "bob" => println!("hello you are the one"),
        "ravi" => println!("hello mister was waitng for you"),
        _ => println!("nice to meet you"),
    }
}

// this is entry point of this program
fn main() {
    // first hello world program as in language
    // println!("Hello, world!");

    // let a = 12;
    // println!("{:?}", a);

    a1::complete_name();

    println!("{:?}", sum(5, 3));

    // simple flow control
    let age = 21;
    if age >= 24 {
        println!("you are not allowed inside");
    } else {
        println!("you are allowed inside");
    }
    // println!("{:?}", c_flow());
    println!("\n");
    f_match();
}
