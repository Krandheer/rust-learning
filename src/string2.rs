fn concate_strings() {
    let mut s1 = String::from("hello");
    let s2 = String::from(" world");
    let s3 = s1.push_str(&s2); // if & was not used then basically s2 will be used and will not
    // availbale in next line.
    println!("s2 is: {s2}");
}

fn main() {
    concate_strings();
}
