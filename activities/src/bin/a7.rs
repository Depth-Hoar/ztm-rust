// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Color {
    Red,
    Black,
    Blue
}

fn color_chooser(color: Color) {
    match color {
        Color::Red => println!("red"),
        Color::Black => println!("black"),
        Color::Blue => println!("blue"),
    }
}

fn main() {
    color_chooser(Color::Red);
}
