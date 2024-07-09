// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum Color {
    Red,
    Green,
    Blue,
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}
impl Dimensions {
    fn print(&self) {
        println!(
            "Width: {}\nHeight: {}\nDepth: {}",
            self.width, self.height, self.depth
        );
    }
}

struct Box {
    dimensions: Dimensions,
    weight: f64,
    color: Color,
}
impl Box {
    fn new(dimensions: Dimensions, weight: f64, color: Color) -> Self {
        Self {
            dimensions,
            weight,
            color,
        }
    }

    fn print(&self) {
        let color = match self.color {
            Color::Blue => "blue",
            Color::Green => "green",
            Color::Red => "red",
        };
        self.dimensions.print();
        println!("Weight: {}\nColor: {}", self.weight, color)
    }
}

fn main() {
    let my_dimensions = Dimensions {
        width: 10.0,
        height: 5.0,
        depth: 1.5,
    };
    let my_box = Box::new(my_dimensions, 10.0, Color::Green);
    my_box.print();
}
