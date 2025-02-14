// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum BoxColor {
    Red,
    Black,
    Blue,
}

impl BoxColor {
    fn print(&self) {
        match self {
            BoxColor::Red => println!("red"),
            BoxColor::Black => println!("black"),
            BoxColor::Blue => println!("blue"),
        }
    }
}

struct BoxType {
    length: i32,
    width: i32,
    height: i32,
    weight: i32,
    color: BoxColor,
}

impl BoxType {
    fn new_box(length: i32, width: i32, height: i32, weight: i32, color: BoxColor) -> Self {
        Self {
            length, 
            width, 
            height, 
            weight, 
            color
        }
    }
    
    fn print(&self) {
        println!("length: {:?}", self.length);
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("weight: {:?}", self.weight);
        self.color.print()
    }
}

fn main() {
    let small_box = BoxType::new_box(3, 2, 4, 7, BoxColor::Red);
    small_box.print();
}
