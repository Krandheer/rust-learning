#[derive(Debug)]
enum Position {
    Manager,
    Supervisor,
    Worker,
}
struct LineItem {
    name: String,
    count: i32,
}
fn some_integer(mut some_integer: i32) {
    // now here some_integer is passed and can be used till when it does not goes out of scope
    println!("some integers: {some_integer}");
    some_integer = some_integer + 1;
    println!("some integers: {some_integer}");
}

fn some_strings(some_string: String) {
    println!("some string: {some_string}");
}
fn main() {
    let receipt = vec![
        LineItem {
            name: "cereal".to_owned(),
            count: 21,
        },
        LineItem {
            name: "fruits".to_owned(),
            count: 11,
        },
        LineItem {
            name: "vegetables".to_owned(),
            count: 31,
        },
        LineItem {
            name: "potatoes".to_owned(),
            count: 61,
        },
    ];
    // for item in receipt {
    //     println!("name: {:?} and number: {:?}", item.name, item.count);
    // }

    println!("");
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("s: {s}, ");
    let p = &s;
    let q = &s;
    println!("this is p: {p} and this is q: {q}");
    let r = &mut s; // after declaring mutable variable to r the scope of p and q ends; 
    println!("r is r: {r}");
}
