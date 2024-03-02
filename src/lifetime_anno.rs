fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    } else {
        return y;
    }
}

fn main() {
    let s1 = String::from("hello_world");
    let s2 = "helloman";
    let longest = longest(s1.as_str(), s2);
    print!("{longest}\n");
}
