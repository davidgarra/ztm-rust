// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.

use std::io::stdin;

mod menu {
    pub enum Item {
        AddBill = 1,
        ShowBills,
    }
    impl Item {
        pub fn from_str(value: &str) -> Option<Self> {
            match value {
                "1" => Some(Item::AddBill),
                "2" => Some(Item::ShowBills),
                _ => None,
            }
        }
    }

    pub fn show() {
        println!("\n--- Interactive Bill Manager ---\n");
        println!("Make a choice:");
        println!("1) Add a bill");
        println!("2) Show bills");
    }
}

#[derive(Debug)]
struct Bill {
    name: String,
    amount: f64,
}

fn read_line() -> String {
    let mut buf = String::new();
    while stdin().read_line(&mut buf).is_err() {
        println!("Please, try again");
    }
    buf.trim().to_owned()
}

fn read_amount() -> f64 {
    loop {
        let Ok(value) = read_line().parse::<f64>() else {
            continue;
        };
        return value;
    }
}

fn add_bill(bills: &mut Vec<Bill>) {
    println!("Insert name: ");
    let name = read_line();
    println!("Insert amount: ");
    let amount = read_amount();
    let bill = Bill { name, amount };
    println!("Adding {:?} to the bills", bill);
    bills.push(bill);
}

fn show_bills(bills: &mut Vec<Bill>) {
    println!("List of bills({}):", bills.len());
    for item in bills {
        println!("{:?}", item);
    }
}

fn run() {
    let mut bills: Vec<Bill> = vec![];
    loop {
        use menu::Item;
        menu::show();
        let input = read_line();
        let Some(choice) = Item::from_str(input.as_str()) else {
            println!("Unknown command");
            continue;
        };
        match choice {
            Item::AddBill => add_bill(&mut bills),
            Item::ShowBills => show_bills(&mut bills),
        }
    }
}

fn main() {
    run();
}
