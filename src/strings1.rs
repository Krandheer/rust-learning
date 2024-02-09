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
    // println!("s: {s}, s_len={Len(s)}");
    let mut x = 5;
    some_strings(s);
    // some_integer(x);
    println!("x: {x}");

    // this below loop throws error because receipt vector got used once and hence doesn't exist in
    // memeory
    // for item in receipt {
    //     println!("name: {:?} and number: {:?}", item.name, item.count);
    // }
}
