// Lesson 16 — Strings: `&str` and `String`
//
// `&str` is a borrowed, immutable view of some text (often a literal).
// `String` is an owned, growable, heap-allocated UTF-8 string.
fn main() {
    // Fixed labels: `&str` literals are cheap and don't need to be owned.
    let label_name: &str = "PASSENGER NAME";
    let label_dest: &str = "DESTINATION";
    let label_note: &str = "NOTE";

    // Owned `String` values for data that is "ours" at runtime.
    let passenger_name: String = String::from("Alice Nguyen");

    // Another way to build a String — from a literal.
    let mut destination: String = "San Francisco".to_string();
    // Append to it. Only possible because `destination` is `mut` AND a String.
    destination.push_str(", CA");

    let note: String = String::from("VIP traveler — priority boarding");

    println!("=== Passenger Record ===");
    // When we print `passenger_name` (a String), it is auto-borrowed as &str.
    println!("{} : {}", label_name, passenger_name);
    println!("{}    : {}", label_dest, destination);
    println!("{}           : {}", label_note, note);
}
