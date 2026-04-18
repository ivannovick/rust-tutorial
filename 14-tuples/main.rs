// Lesson 14 — Tuples
//
// A tuple groups several values of possibly different types. It's handy
// when a function needs to return more than one value but a full struct
// would be overkill.
fn main() {
    // Call the function; it returns a 3-tuple.
    let summary = flight_summary();

    // Destructure the tuple into three named bindings.
    let (number, destination, departure_time) = summary;

    println!("Flight {} -> {} at {}", number, destination, departure_time);
}

// Return type: a tuple of three string slices.
fn flight_summary() -> (&'static str, &'static str, &'static str) {
    // Last expression, no semicolon: it's the returned value.
    ("AA1234", "San Francisco", "10:30")
}
