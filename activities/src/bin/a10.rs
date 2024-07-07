// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn main() {
    print_message(is_greater_than_100(10));
    print_message(is_greater_than_100(100));
    print_message(is_greater_than_100(1000));
}

fn is_greater_than_100(value: i32) -> bool {
    if value > 100 {
        true
    } else {
        false
    }
}

fn print_message(is_greater: bool) {
    match is_greater {
        true => println!("it's big"),
        false => println!("it's small"),
    }
}
