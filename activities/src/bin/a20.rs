// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

use std::io::{self, stdin};

enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

impl PowerState {
    fn new(power_state: &str) -> Result<Self, &str> {
        use PowerState::*;
        match power_state.to_lowercase().as_str() {
            "off" => Ok(Off),
            "sleep" => Ok(Sleep),
            "reboot" => Ok(Reboot),
            "shutdown" => Ok(Shutdown),
            "hibernate" => Ok(Hibernate),
            _ => Err("Unknown state"),
        }
    }
    fn print(&self) {
        use PowerState::*;
        match self {
            Off => println!("turning off"),
            Sleep => println!("sleeping"),
            Reboot => println!("rebooting"),
            Shutdown => println!("shutting down"),
            Hibernate => println!("hiberneting"),
        }
    }
}

fn input_line() -> io::Result<String> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}

fn print_state(state: &str) {
    let power_state = PowerState::new(state);
    match power_state {
        Ok(power_state) => power_state.print(),
        Err(error) => println!("{error}"),
    }
}

fn main() {
    loop {
        let line = input_line();
        match line {
            Err(error) => println!("{error}"),
            Ok(input) => print_state(&input),
        }
    }
}
