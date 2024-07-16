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

#[derive(Debug, Clone)]
struct Bill {
    name: String,
    amount: f64,
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

    fn add(&mut self, name: &str, amount: f64) -> Option<Bill> {
        let bill = Bill {
            name: name.to_string(),
            amount,
        };
        self.bills.insert(name.to_string(), bill)
    }

    fn remove(&mut self, name: &str) -> Option<Bill> {
        self.bills.remove(name)
    }

    fn update(&mut self, name: &str, amount: f64) -> Option<Bill> {
        let Some(bill) = self.bills.get_mut(name) else {
            return None;
        };

        bill.amount = amount;
        Some(bill.clone())
    }

    fn get_all(&self) -> Vec<Bill> {
        self.bills.values().cloned().collect()
    }
}

mod menu {
    use crate::{read_f64, read_string, Bills};

    pub enum Item {
        AddBill = 1,
        ShowBills,
        RemoveBills,
        EditBills,
    }
    impl Item {
        pub fn from_str(value: &str) -> Option<Self> {
            match value {
                "1" => Some(Item::AddBill),
                "2" => Some(Item::ShowBills),
                "3" => Some(Item::RemoveBills),
                "4" => Some(Item::EditBills),
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
        println!("4) Edit bill");
    }

    pub fn add_bill(bills: &mut Bills) {
        println!("Insert name: ");
        let Some(name) = read_string() else {
            return;
        };
        println!("Insert amount: ");
        let amount = match read_f64() {
            Some(amount) => amount,
            None => return,
        };
        println!("Adding bill with name {name} and amount {amount}");
        match bills.add(&name, amount) {
            Some(_) => println!("Bill added successfully"),
            None => println!("Something went wrong"),
        };
    }

    pub fn remove_bill(bills: &mut Bills) {
        println!("Insert name: ");
        let Some(name) = read_string() else {
            return;
        };
        println!("Removing bill with name {name}");
        match bills.remove(&name) {
            Some(_) => println!("Bill removed successfully"),
            None => println!("Bill not found"),
        }
    }

    pub fn edit_bill(bills: &mut Bills) {
        println!("Insert name: ");
        let Some(name) = read_string() else {
            return;
        };

        println!("Insert amount: ");
        let Some(amount) = read_f64() else {
            return;
        };

        println!("Updating bill with name {name}");
        match bills.update(&name, amount) {
            Some(_) => println!("Bill updated successfully"),
            None => println!("Bill not found"),
        }
    }

    pub fn list_bills(bills: &Bills) {
        let list = bills.get_all();
        println!("List of bills({})", list.len());
        for bill in list {
            println!("{:?}", bill);
        }
    }
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
        let Some(input) = read_string() else {
            continue;
        };
        let Some(choice) = Item::from_str(input.as_str()) else {
            println!("Unknown command");
            continue;
        };
        match choice {
            Item::AddBill => menu::add_bill(&mut bills),
            Item::ShowBills => menu::list_bills(&bills),
            Item::RemoveBills => menu::remove_bill(&mut bills),
            Item::EditBills => menu::edit_bill(&mut bills),
        }
    }
}

fn main() {
    run();
}
