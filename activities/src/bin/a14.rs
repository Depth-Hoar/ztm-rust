// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    name: String,
    age: i32,
    fav_color: String,
}

fn print(data: &str) {
    println!("{:?}", data);
}

fn main() {
    let people = vec![
        Person { 
            name: String::from("Alice"),
            age: 5,
            fav_color: String::from("pink"),
        },
        Person { 
            name: String::from("Bob"),
            age: 7,
            fav_color: String::from("red"),
        },
        Person { 
            name: String::from("Jill"),
            age: 14,
            fav_color: String::from("yellow"),
        }, 
    ];

    for person in people {
        if person.age <= 10 {
            print(&person.name);
            print(&person.fav_color);
        }
    }
}
