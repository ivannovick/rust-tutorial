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
    if status == "on_time" {
        println!("  --> On time. Boarding will begin shortly.");
    } else if status == "delayed" {
        println!("  --> Delayed. Please check the departure board for updates.");
    } else if status == "cancelled" {
        println!("  --> Cancelled. Please contact the rebooking desk.");
    } else {
        println!("  --> Unknown status.");
    }
}
