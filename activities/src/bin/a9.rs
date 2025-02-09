// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn coordinate(x:i32, y:i32) -> (i32, i32) {
    (x,y)
}

fn main() {
    let location = coordinate(4,7);

    if location.1 > 5 {
        println!("y > 5")
    } else if location.1 < 5 {
        println!("y < 5")
    } else {
        println!("y = 5")
    }

}
