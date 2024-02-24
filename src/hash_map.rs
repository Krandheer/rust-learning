use std::collections::HashMap;
fn main() {
    let mut hash_map = HashMap::new();
    hash_map.insert("ravi", 21);
    hash_map.insert("sunita", 31);
    hash_map.insert("randheer", 29);
    hash_map.insert("ambey", 25);
    for (key, val) in hash_map.iter() {
        println!("key: {:?}, value: {:?}", key, val);
    }
    for (key, val) in &hash_map {
        println!("{key}: {val}");
    }
    println!("{:?}", hash_map);

    println!("");
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // not able to understand this line
        *count += 1;
    }

    println!("{:?}", map);
}
