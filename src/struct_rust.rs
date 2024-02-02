struct ShippingBox {
    height: i32,
    width: i32,
    depth: i32,
}
fn main() {
    let my_box = ShippingBox {
        height: 3,
        width: 4,
        depth: 5,
    };

    let tall = my_box.height;
    println!("{:?}", tall);
}
