// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn main() {
    let (_, y) = get_coordinate(1, 1);
    print_value(y);
    let (_, y) = get_coordinate(1, 10);
    print_value(y);
    let (_, y) = get_coordinate(1, 5);
    print_value(y);
    let (_, y) = get_coordinate(2, 1);
    print_value(y);
}

fn get_coordinate(x: i32, y: i32) -> (i32, i32) {
    (x, y)
}

fn print_value(value: i32) {
    if value > 5 {
        println!(">5");
    } else if value < 5 {
        println!("<5");
    } else {
        println!("=5");
    }
}
