// Lesson 02 — Variables with `let`
//
// Use `let` to create a new variable binding. By default, bindings are
// immutable — their value cannot be changed after the initial assignment.
fn main() {
    // Three immutable bindings that hold airline booking details.
    let passenger_name = "Alice Nguyen";
    let flight_number = "AA1234";
    let destination = "San Francisco";

    println!("=== Booking Summary ===");
    // `{}` is a placeholder that is replaced by the matching argument.
    println!("Passenger:   {}", passenger_name);
    println!("Flight:      {}", flight_number);
    println!("Destination: {}", destination);
}
