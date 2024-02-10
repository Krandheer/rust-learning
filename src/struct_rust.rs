struct ShippingBox {
    height: i32,
    width: i32,
    depth: i32,
}

#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        return self.height * self.width;
    }
}
fn main() {
    let my_box = ShippingBox {
        height: 3,
        width: 4,
        depth: 5,
    };

    // let tall = my_box.height;
    // println!("{:?}", tall);
    let rectangle = Rectangle {
        height: 12,
        width: 4,
    };
    // let rect_area = area(&rectangle);
    println!("rectangle area is: {:?}", rectangle.area());
    println!("rectangle is: {:#?}", rectangle);
}

fn area(s: &Rectangle) -> u32 {
    return s.height * s.width;
}
