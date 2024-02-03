#[derive(Debug)]
enum Position {
    Manager,
    Supervisor,
    Worker
}
struct LineItem {
    name: String,
    count: i32,
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
    for item in receipt {
        println!("name: {:?} and number: {:?}", item.name, item.count);
    }

    println!("");
    // this below loop throws error because receipt vector got used once and hence doesn't exist in
    // memeory
    // for item in receipt {
    //     println!("name: {:?} and number: {:?}", item.name, item.count);
    // }
}
