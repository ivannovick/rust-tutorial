// Lesson 12 — Functions with return values
//
// The return type is written after `->`. The last expression in the body
// (with NO semicolon) is the returned value.
fn main() {
    let bags = 2;

    // Call each helper and combine their results.
    let fare = base_fare();
    let bag_fee = baggage_charge(bags);
    let ticket_total = total(fare, bag_fee);

    println!("Base fare:      ${:.2}", fare);
    println!("Baggage ({} bags): ${:.2}", bags, bag_fee);
    println!("Total:          ${:.2}", ticket_total);
}

// Returns an f64. No semicolon on the final line => that's the return value.
fn base_fare() -> f64 {
    249.99
}

// $30 per checked bag.
fn baggage_charge(num_bags: i32) -> f64 {
    30.00 * (num_bags as f64) // cast i32 to f64 so the math works
}

// Sum of two f64 values.
fn total(fare: f64, baggage: f64) -> f64 {
    fare + baggage
}
