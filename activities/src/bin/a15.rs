// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Ticket {
    Backstage(f32, String),
    Standard(f32),
    Vip(f32, String),
}

fn main() {
    let tickets = vec![
        Ticket::Backstage(50.0, "Alice".to_owned()),
        Ticket::Standard(20.0),
        Ticket::Vip(100.0, "Bob".to_owned()),
    ];
    for ticket in tickets {
        match ticket {
            Ticket::Backstage(pice, holder) => println!("Backstage price: {:?}, Holder: {:?}", pice, holder),
            Ticket::Standard(pice) => println!("Standard price: {:?}", pice),
            Ticket::Vip(pice, holder) => println!("VIP price: {:?}, Holder: {:?}", pice, holder),
        }
    }
}
