// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct Item {
    quantity: i32,
    id: i32,
}

fn print_quant (item: &Item) {
    println!("quantity: {:?}", item.quantity);
}

fn print_id (item: &Item) {
    println!("ID: {:?}", item.id);
}

fn main() {
    let bananas = Item {
        quantity: 4,
        id: 4857398,
    };
    print_quant(&bananas);
    print_id(&bananas);
}
