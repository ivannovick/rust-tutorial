// Lesson 13 — Arrays
//
// `[T; N]` is an array of N elements of type T. Size is fixed at compile
// time. Great for small, known-size collections.
fn main() {
    // No type annotation needed — the compiler infers `[&str; 7]`
    // from the literal: 7 string slices => length 7, element type &str.
    let departing_flights = [
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

    // A second array — this time of integers. Again no annotation:
    // the compiler infers `[i32; 20]` (20 integer literals => length 20,
    // integer default => i32). These are the 20 active gate numbers in
    // Terminal B.
    let active_gates = [
        1,  2,  3,  4,  5,
        7,  8,  9, 11, 12,
       14, 15, 17, 18, 19,
       20, 22, 24, 25, 27,
    ];

    println!();
    println!("=== 20 Active Gates in Terminal B ===");
    for gate in active_gates {
        print!("{} ", gate);
    }
    println!(); // terminate the line after the last gate

    // Arrays know their own length at compile time.
    println!("(total gates: {})", active_gates.len());
}
