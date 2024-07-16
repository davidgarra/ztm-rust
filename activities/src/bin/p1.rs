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

use std::{collections::HashMap, io::stdin};

mod menu {
    pub enum Item {
        AddBill = 1,
        ShowBills,
        RemoveBills,
    }
    impl Item {
        pub fn from_str(value: &str) -> Option<Self> {
            match value {
                "1" => Some(Item::AddBill),
                "2" => Some(Item::ShowBills),
                "3" => Some(Item::RemoveBills),
                _ => None,
            }
        }
    }

    pub fn show() {
        println!("\n--- Interactive Bill Manager ---\n");
        println!("Make a choice:");
        println!("1) Add a bill");
        println!("2) Show bills");
        println!("3) Remove bill");
    }
}

struct Bills {
    bills: HashMap<String, Bill>,
}

impl Bills {
    fn new() -> Bills {
        Bills {
            bills: HashMap::new(),
        }
    }

    fn add(&mut self) {
        println!("Insert name: ");
        let name = match read_string() {
            Some(line) => line,
            None => return,
        };
        println!("Insert amount: ");
        let amount = match read_f64() {
            Some(amount) => amount,
            None => return,
        };
        let bill = Bill {
            name: name.clone(),
            amount,
        };
        println!("Adding {:?} to the bills", bill);
        self.bills.insert(name, bill);
    }

    fn remove(&mut self) {
        println!("Insert name: ");
        let name = match read_string() {
            Some(line) => line,
            None => return,
        };
        println!("Removing {name} from the bills");
        if let Some(bill) = self.bills.remove(&name) {
            println!("Bill {:?} removed", bill);
        } else {
            println!("Bill not found");
        }
    }

    fn show(&self) {
        println!("List of bills({}):", self.bills.len());
        for item in &self.bills {
            println!("{:?}", item);
        }
    }
}

#[derive(Debug)]
struct Bill {
    name: String,
    amount: f64,
}

fn read_string() -> Option<String> {
    let mut buf = String::new();
    while stdin().read_line(&mut buf).is_err() {
        println!("Please, try again");
    }
    let line = buf.trim();
    match line {
        "" => None,
        _ => Some(line.to_owned()),
    }
}

fn read_f64() -> Option<f64> {
    loop {
        let line = match read_string() {
            None => return None,
            Some(line) => line,
        };
        let amount: Result<f64, _> = line.parse();
        match amount {
            Ok(amount) => return Some(amount),
            Err(_) => println!("Please, enter a number"),
        }
    }
}

fn run() {
    let mut bills: Bills = Bills::new();
    loop {
        use menu::Item;
        menu::show();
        let input = match read_string() {
            Some(value) => value,
            None => continue,
        };
        let Some(choice) = Item::from_str(input.as_str()) else {
            println!("Unknown command");
            continue;
        };
        match choice {
            Item::AddBill => bills.add(),
            Item::ShowBills => bills.show(),
            Item::RemoveBills => bills.remove(),
        }
    }
}

fn main() {
    run();
}
