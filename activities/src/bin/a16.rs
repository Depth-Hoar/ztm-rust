// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Locker {
    name: String,
    number: Option<i32>,
}

fn main() {
    let dave = Locker {
        name: "Dave".to_owned(),
        number: Some(15),
    };
    println!("Student: {:?}", dave.name);
    match dave.number {
        Some(n) => println!("Locker Number: {:?}", n),
        _ => println!("No Locker"),
    };
}
