struct Test {
    run: i32,
}
fn main() {
    // let mut my_number = vec![1, 2, 3];
    // my_number.push(4);
    // println!("{:?}", my_number);
    // for num in my_number {
    //     println!("{:?}", num);
    // }
    let scores = vec![
        Test { run: 100 },
        Test { run: 90 },
        Test { run: 84 },
        Test { run: 98 },
        Test { run: 10 },
    ];
    for run in scores {
        println!("{:?}", run.run);
    }
}
