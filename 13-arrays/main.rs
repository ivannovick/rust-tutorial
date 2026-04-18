// Lesson 13 — Arrays
//
// `[T; N]` is an array of N elements of type T. Size is fixed at compile
// time. Great for small, known-size collections.
fn main() {
    // An array of 7 string slices. The type is `[&str; 7]`.
    let departing_flights: [&str; 7] = [
        "AA1234",
        "UA5678",
        "DL9012",
        "WN3456",
        "AS7890",
        "B61122",
        "NK3344",
    ];

    println!("=== Next 7 Departures ===");

    // `iter().enumerate()` gives us both the index and the value.
    // We add 1 to the index so the display starts at 1 instead of 0.
    for (i, flight) in departing_flights.iter().enumerate() {
        println!("{}. {}", i + 1, flight);
    }
}
