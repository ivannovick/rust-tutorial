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

fn main() {
    // Construct three flights. Field order in the literal doesn't matter,
    // but every field must be set.
    let f1 = Flight {
        number: String::from("AA1234"),
        destination: String::from("San Francisco"),
        gate: 17,
        seats_available: 42,
    };

    let f2 = Flight {
        number: String::from("UA5678"),
        destination: String::from("Chicago"),
        gate: 22,
        seats_available: 8,
    };

    let f3 = Flight {
        number: String::from("DL9012"),
        destination: String::from("Atlanta"),
        gate: 3,
        seats_available: 110,
    };

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
