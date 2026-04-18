// Lesson 21 — `Option` and `Result`
//
// `Option<T>` models "a value that might be missing".
// `Result<T, E>` models "an operation that might fail".
// Together they replace null and exceptions.
use std::num::ParseFloatError;

fn main() {
    // --- Option example ------------------------------------------------
    for name in ["Alice", "Zoe"] {
        println!("Searching for passenger '{}'...", name);
        match find_seat(name) {
            Some(seat) => println!("  Seat found: {}", seat),
            None       => println!("  No seat assigned."),
        }
    }

    println!();

    // --- Result example ------------------------------------------------
    for input in ["23.5", "heavy"] {
        println!("Parsing baggage weight '{}'...", input);
        match parse_weight(input) {
            Ok(w)  => println!("  Weight: {:.2} kg", w),
            Err(e) => println!("  Parse error: {}", e),
        }
    }
}

// Returns `Some(seat)` if the passenger is known, else `None`.
fn find_seat(name: &str) -> Option<u32> {
    match name {
        "Alice" => Some(14),
        "Bob"   => Some(22),
        _       => None, // everyone else: no seat on file
    }
}

// Parse a weight string into an f64. `str::parse()` returns a Result,
// which we return directly.
fn parse_weight(input: &str) -> Result<f64, ParseFloatError> {
    input.parse::<f64>()
}
