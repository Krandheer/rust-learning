pub(crate) fn learn_variables() {
    // let mut x = 5;
    // println!("the value of x is: {x}");
    // x = 6;
    // println!("the value of x is: {x}");

    // scoping and shadoing of variable
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("the value of x in inner scope is {x}");
    }
    println!("value of x is: {x}");
}
