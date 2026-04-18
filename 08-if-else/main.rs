// Lesson 08 — `if` and `else`
//
// `if` is an expression in Rust. The condition must evaluate to `bool`.
// Use `else if` for multi-way branching.
fn main() {
    // Check three different flights, each in a different state.
    report_status("AA1234", "on_time");
    report_status("UA5678", "delayed");
    report_status("DL9012", "cancelled");
}

fn report_status(flight_number: &str, status: &str) {
    println!("Flight {} status: {}", flight_number, status);

    // Multi-way branching with `else if`. Note: we compare strings with `==`.
    // Each branch includes the flight number so the message is self-contained.
    if status == "on_time" {
        println!("  --> Flight {}: On time. Boarding will begin shortly.", flight_number);
    } else if status == "delayed" {
        println!("  --> Flight {}: Delayed. Please check the departure board for updates.", flight_number);
    } else if status == "cancelled" {
        println!("  --> Flight {}: Cancelled. Please contact the rebooking desk.", flight_number);
    } else {
        println!("  --> Flight {}: Unknown status.", flight_number);
    }
}
