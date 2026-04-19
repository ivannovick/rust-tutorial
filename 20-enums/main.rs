// Lesson 20 — Enums
//
// An `enum` defines a type whose value is exactly one of several named
// variants. It's the type-safe way to model "one-of" choices.
enum BookingStatus {
    Confirmed,
    Standby,
    CheckedIn,
    Boarded,
}

fn main() {
    // Each passenger has a name and a booking status.
    let passengers: [(&str, BookingStatus); 4] = [
        ("Alice Nguyen",  BookingStatus::CheckedIn),
        ("Bob Patel",     BookingStatus::Confirmed),
        ("Carol Singh",   BookingStatus::Standby),
        ("Dan O'Reilly",  BookingStatus::Boarded),
    ];

    // Iterate by reference so we don't move items out of the array.
    for (name, status) in &passengers {
        println!("{:<14} {}", format!("{}:", name), label(status));
    }
}

// Turn a BookingStatus into a human-readable string.
// `match` (covered fully in Lesson 22) is the idiomatic way to do this.
fn label(status: &BookingStatus) -> &'static str {
    match status {
        BookingStatus::Confirmed => "Confirmed",
        BookingStatus::Standby   => "Standby",
        BookingStatus::CheckedIn => "Checked In",
        BookingStatus::Boarded   => "Boarded",
    }
}
