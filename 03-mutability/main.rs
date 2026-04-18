// Lesson 03 — Mutability with `mut`
//
// A `let` binding is immutable by default. Use `let mut` to make it mutable
// so its value can change over time. In this airline example, seat count
// drops as bookings are confirmed.
fn main() {
    // `mut` allows us to reassign a new value to `seats_available` later.
    let mut seats_available = 10;
    let flight_number = "AA1234"; // stays the same — no `mut` needed

    println!(
        "Starting with {} seats available on flight {}",
        seats_available, flight_number
    );

    // Simulate three bookings. Each one reduces seat availability by 1.
    seats_available = seats_available - 1;
    println!("Booking 1 confirmed — seats available: {}", seats_available);

    seats_available -= 1; // shorthand for seats_available = seats_available - 1
    println!("Booking 2 confirmed — seats available: {}", seats_available);

    seats_available -= 1;
    println!("Booking 3 confirmed — seats available: {}", seats_available);

    println!("Final seat count: {}", seats_available);
}
