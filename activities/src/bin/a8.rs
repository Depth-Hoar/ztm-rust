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
    Grape,
    Orange,
    Strawberry
}

struct DrinkType {
    flavor: Flavor,
    ounces: f64,
}

fn print_drink(drinkType: DrinkType) {
    match drinkType.flavor {
        Flavor::Grape => println!("grape"),
        Flavor::Orange => println!("orange"),
        Flavor::Strawberry => println!("strawberry"),
    }

    println!("ounces: {:?}", drinkType.ounces);
}

fn main() {
    let strawberry = DrinkType {
        flavor: Flavor::Strawberry,
        ounces: 6.0
    };

    print_drink(strawberry);
}
