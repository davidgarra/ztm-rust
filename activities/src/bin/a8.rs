// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    Sparkling,
    Sweet,
    Fruity,
}

struct Drink {
    flavor: Flavor,
    oz: f64,
}

fn main() {
    print_drink(Drink {
        flavor: Flavor::Fruity,
        oz: 10f64,
    });
    print_drink(Drink {
        flavor: Flavor::Sparkling,
        oz: 8.4,
    });
    print_drink(Drink {
        flavor: Flavor::Sweet,
        oz: 0.1,
    });
}

fn print_drink(drink: Drink) {
    let drink_flavor = match drink.flavor {
        Flavor::Fruity => "fruity",
        Flavor::Sparkling => "sparkling",
        Flavor::Sweet => "sweet",
    };
    println!("{}: {}", drink_flavor, drink.oz)
}
