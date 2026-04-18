// Lesson 04 — Basic data types
//
// Rust is statically typed. Each value has a type determined at compile
// time. This program stores flight info using several common primitives.
fn main() {
    let flight_number: &str = "AA1234";   // &str  — string slice (borrowed text)
    let gate_number: i32 = 17;            // i32   — 32-bit signed integer
    let ticket_price: f64 = 249.99;       // f64   — 64-bit floating point
    let on_time: bool = true;             // bool  — true or false
    let boarding_zone: char = 'B';        // char  — one Unicode scalar value

    println!("=== Flight Info ===");
    println!("Flight:        {}", flight_number);
    println!("Gate:          {}", gate_number);
    // `:.2` formats a float with 2 decimal places.
    println!("Ticket price:  ${:.2}", ticket_price);
    println!("On time:       {}", on_time);
    println!("Boarding zone: {}", boarding_zone);
}
