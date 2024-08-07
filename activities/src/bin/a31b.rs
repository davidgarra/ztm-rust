// Topic: Trait Objects
//
// Summary:
//   A zoo wants a program that can simulate the sounds made by different animals.
//
// Requirements:
// * Simulate multiple animal types with different sounds.
// * Must be able to process a list of varying animals.
// * Animal types and sounds include:
//   * Dog - "Bark"
//   * Cat - "Meow"
//   * Cow - "Moo"
// * The sound each animal makes must be taken into account.

//
// Notes:
// * Create a trait that can be used to retrieve the sound of an animal.
// * Create trait objects and store them in a vector for processing.
// * Use a function to print the sounds of all animals.
// * Process at least 3 different animals.

use std::vec;

trait Animal {
    fn sound(&self) -> String;
}

struct Dog;
impl Animal for Dog {
    fn sound(&self) -> String {
        String::from("Bark")
    }
}

struct Cat;
impl Animal for Cat {
    fn sound(&self) -> String {
        String::from("Meow")
    }
}
struct Cow;
impl Animal for Cow {
    fn sound(&self) -> String {
        String::from("Moo")
    }
}

fn print_all(animals: Vec<Box<dyn Animal>>) {
    animals
        .iter()
        .for_each(|animal| println!("{}", animal.sound()));
}

fn main() {
    let dog = Box::new(Dog);
    let cat = Box::new(Cat);
    let cow = Box::new(Cow);
    let animals: Vec<Box<dyn Animal>> = vec![dog, cat, cow];
    print_all(animals)
}
