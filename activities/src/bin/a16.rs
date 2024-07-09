// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    name: String,
    locker: Option<i32>,
}

impl Student {
    fn print_locker(&self) {
        match self.locker {
            Some(n) => println!("{} locker: {}", self.name, n),
            None => println!("No locker assigned to {}", self.name),
        }
    }
}

fn main() {
    let david = Student {
        name: "David".to_owned(),
        locker: Some(10),
    };
    let vanessa = Student {
        name: "Vanessa".to_owned(),
        locker: None,
    };
    david.print_locker();
    vanessa.print_locker();
}
