// Lesson 23 — Ownership, borrowing, and memory basics
//
// Rust enforces memory safety at compile time via ownership.
//   - Each value has one owner.
//   - Passing a value to a function normally MOVES ownership.
//   - Taking `&value` instead BORROWS a reference; ownership stays put.
fn main() {
    let passenger_name: String = String::from("Alice Nguyen");

    // Borrow (not move): pass `&passenger_name`. The function gets a
    // read-only reference; we still own the String afterwards.
    print_name(&passenger_name);

    // Still allowed — `passenger_name` was only borrowed.
    println!("Name is still usable here: {}", passenger_name);

    let booking_note: String = String::from("VIP traveler — priority boarding");

    // Move: pass `booking_note` directly. Ownership transfers into
    // `take_note`. After this call, we can NOT use `booking_note` again.
    take_note(booking_note);

    // println!("{}", booking_note);  // <- would fail to compile: value moved
}

// Borrows a String read-only.
fn print_name(name: &String) {
    println!("Printing name (borrowed): {}", name);
    // Because `name` is a reference, this function cannot modify or drop
    // the original — the caller still owns it.
}

// Takes ownership. When this function ends, `note` is dropped (freed).
fn take_note(note: String) {
    println!("Recorded booking note: {}", note);
}
