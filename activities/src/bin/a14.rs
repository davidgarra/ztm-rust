// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

#[derive(Debug)]
struct Person {
    age: i32,
    name: String,
    color: String,
}

impl Person {
    fn new(name: String, age: i32, color: String) -> Self {
        Person { name, age, color }
    }
}

fn print(value: &str) {
    println!("{}", value)
}

fn main() {
    let people = vec![
        Person::new(String::from("David"), 9, String::from("red")),
        Person::new(String::from("Pamela"), 15, String::from("blue")),
        Person::new(String::from("Sara"), 8, String::from("green")),
    ];

    for person in people {
        if person.age <= 10 {
            print(&person.name);
            print(&person.color);
            println!("{:?}\n", person);
        }
    }
}
