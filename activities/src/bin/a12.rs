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

struct Box {
    dimensions: (i32, i32, i32),
    weight: f64,
    color: Color,
}

impl Box {
    fn new(dimensions: (i32, i32, i32), weight: f64, color: Color) -> Self {
        Self {
            dimensions,
            weight,
            color,
        }
    }

    fn print(&self) {
        let color = match &self.color {
            Color::Blue => "blue",
            Color::Green => "green",
            Color::Red => "red",
        };
        println!(
            "Dimensions: {:?}\n Weight: {}\n Color: {}",
            &self.dimensions, &self.weight, color
        )
    }
}

fn main() {
    let my_box = Box::new((1, 0, 3), 10.0, Color::Green);
    my_box.print();
}
