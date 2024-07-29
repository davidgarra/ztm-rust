// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter

trait Perimeter {
    fn perimeter(&self) -> i32;
}
struct Square {
    side: i32,
}
impl Perimeter for Square {
    fn perimeter(&self) -> i32 {
        self.side * 4
    }
}

struct Triangle {
    sides: (i32, i32, i32),
}
impl Perimeter for Triangle {
    fn perimeter(&self) -> i32 {
        self.sides.0 + self.sides.1 + self.sides.2
    }
}

fn print_perimeter(shape: impl Perimeter) {
    println!("Perimeter: {}", shape.perimeter())
}

fn main() {
    let triangle = Triangle { sides: (10, 3, 2) };
    print_perimeter(triangle);

    let square = Square { side: 4 };
    print_perimeter(square);
}
