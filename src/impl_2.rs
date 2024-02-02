enum Color {
    Red,
    Blue,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Red => println!("red"),
            Color::Blue => println!("blue"),
        }
    }
}

struct Dimension {
    depth: f64,
    height: f64,
    width: f64,
}

impl Dimension {
    fn print(&self) {
        println!("depth is: {:?}", self.depth);
        println!("height is: {:?}", self.height);
        println!("width is: {:?}", self.width);
    }
}

struct ShippingBox {
    color: Color,
    weight: f64,
    dimension: Dimension,
}

impl ShippingBox {
    fn new(weight: f64, color: Color, dimension: Dimension) -> Self {
        Self {
            weight,
            color,
            dimension,
        }
    }
    fn print(&self) {
        println!("weight is: {:?}", self.weight);
        self.color.print();
        self.dimension.print();
    }
}

fn main() {
    let small_dimensions = Dimension {
        depth: 1.0,
        height: 3.0,
        width: 4.0,
    };
    let small_box = ShippingBox::new(5.0, Color::Red, small_dimensions);
    small_box.print()
}
