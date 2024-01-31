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
// this is entry point of this program
fn main() {
    // first hello world program as in language
    // println!("Hello, world!");

    // let a = 12;
    // println!("{:?}", a);

    a1::complete_name();

    println!("{:?}", sum(5, 3));

    let age = 21;
    if age >= 24 {
        println!("you are not allowed inside");
    } else {
        println!("you are allowed inside");
    }
    // println!("{:?}", c_flow());
}
