// Lesson 11 — Functions: basics
//
// `fn` declares a function. Functions can take parameters and return
// values. `main` is just a regular function that Rust calls at startup.
fn main() {
    // Call each helper in order.
    welcome_banner();
    flight_summary("AA1234", "San Francisco", 17);
    boarding_message();
}

// Takes no parameters, returns nothing.
fn welcome_banner() {
    println!("========================================");
    println!("      Welcome to SkyHigh Airlines");
    println!("========================================");
}

// Takes 3 parameters. Parameter types are required on `fn` signatures —
// unlike `let`, there's no inference here.
fn flight_summary(flight: &str, destination: &str, gate: i32) {
    println!("Flight {} to {} — Gate {}", flight, destination, gate);
}

fn boarding_message() {
    println!("Boarding now. Please have your pass ready.");
}
