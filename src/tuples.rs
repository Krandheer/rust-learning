fn tuples_val() -> (i32, i32, i32) {
    return (1, 2, 3);
}
fn main() {
    let nums = tuples_val();
    // println!("{:?}", nums);
    let (x, y, z) = nums; // destructuring of tuples
    println!("x: {:?}", x);
    println!("y: {:?}", y);
    println!("z: {:?}", z);

    let coord = (2, 7);
    println!("coordinates of x and y is:");
    println!("x is: {:?} y is: {:?}", coord.0, coord.1);
}
