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
}
