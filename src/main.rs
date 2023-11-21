struct Box {
    dimensions: Dimensions,
    weight: f64,
    color: Color
}

enum Color {
    Red,
    Blue,
    Green
}

struct  Dimensions {
    width: f64,
    height: f64,
    depth: f64
}

impl Box {
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self { dimensions, weight, color }
    }
    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("weight: {:?}", self.weight);
    }
}

impl Color {
    fn print(&self) {
        match self {
            Color::Red => println!("red"),
            Color::Blue => println!("blue"),
            Color::Green => println!("green")
        }
    }
}

impl Dimensions {
    fn print(&self) {
        println!("Width: {:?}", self.width);
        println!("Height: {:?}", self.height);
        println!("Depth: {:?}", self.depth);
    }
}

fn main() {
    let small_dims = Dimensions {
        width: 10.0, height: 20.0, depth: 30.0
    };
    let small_box = Box::new(40.0, Color::Red, small_dims);
    small_box.print();
}