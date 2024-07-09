// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Ticket {
    Backstage(f64, String),
    Vip(f64, String),
    Standard(f64),
}

fn main() {
    let backstage = Ticket::Backstage(500f64, String::from("David"));
    let standard = Ticket::Standard(15.50);
    let vip = Ticket::Vip(100.0, String::from("Vanessa"));

    let tickets = vec![backstage, standard, vip];

    for t in tickets {
        match t {
            Ticket::Backstage(price, name) => println!("backstage: ({}, {})", price, name),
            Ticket::Standard(price) => println!("standard: {}", price),
            Ticket::Vip(price, name) => println!("vip: ({}, {})", price, name),
        }
    }
}
