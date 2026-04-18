// Lesson 10 — `while` and `for`
//
// `while` runs while a condition holds.
// `for`   iterates over a sequence (range, array, etc.).
fn main() {
    // --- while loop ---------------------------------------------------
    println!("=== Gates (while) ===");
    let mut gate_number = 1;
    while gate_number <= 5 {
        println!("Gate {} is ready", gate_number);
        gate_number += 1;
    }

    println!();

    // --- for loop -----------------------------------------------------
    println!("=== Flights (for) ===");
    // Array of flight numbers; `for flight in flights` iterates each element.
    let flights = ["AA1234", "UA5678", "DL9012", "WN3456"];
    for flight in flights {
        println!("Upcoming flight: {}", flight);
    }
}
