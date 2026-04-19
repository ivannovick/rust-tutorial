// Lesson 19 — Structs
//
// A struct groups related fields into one named type. It's the primary
// way to model domain objects (like a Flight) in Rust.
struct Flight {
    number: String,
    destination: String,
    gate: u32,
    seats_available: u32,
}

// A tiny `impl` block with a constructor. (Full `impl` + methods coverage
// is in Lesson 24 — here we just use it to make construction cleaner.)
//
// Taking &str parameters and converting internally means callers can pass
// plain string literals instead of wrapping each one in String::from(...).
// `Self` is shorthand for the surrounding type — `Flight` in this case.
impl Flight {
    fn new(number: &str, destination: &str, gate: u32, seats_available: u32) -> Self {
        Self {
            number: number.to_string(),
            destination: destination.to_string(),
            // `gate` and `seats_available` are already the right type —
            // no conversion needed, and we use the field-init shorthand
            // (write `gate` once instead of `gate: gate`).
            gate,
            seats_available,
        }
    }
}

fn main() {
    // Types are inferred throughout — `let f1 = Flight::new(...)` gives
    // us a `Flight` without ever writing that type out.
    let f1 = Flight::new("AA1234", "San Francisco", 17, 42);
    let f2 = Flight::new("UA5678", "Chicago",       22,  8);
    let f3 = Flight::new("DL9012", "Atlanta",        3, 110);

    // Borrow each flight when printing so the originals stay usable.
    print_flight(&f1);
    print_flight(&f2);
    print_flight(&f3);
}

// Takes a shared reference (`&Flight`) — we don't need to own or modify it.
fn print_flight(f: &Flight) {
    // `{:<13}` left-aligns the destination in a 13-character column,
    // so the parenthetical columns line up nicely.
    println!(
        "Flight {} -> {:<13} (Gate {}, {} seats)",
        f.number, f.destination, f.gate, f.seats_available
    );
}
