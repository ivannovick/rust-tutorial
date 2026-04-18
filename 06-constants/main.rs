// Lesson 06 — Constants
//
// `const` declares a value known at compile time. Constants are typed,
// immutable, and typically written in SCREAMING_SNAKE_CASE. They can live
// at module scope (outside of any function), visible to everything below.
const BAGGAGE_FEE_USD: f64 = 30.0; // flat bag fee
const TAX_RATE: f64 = 0.075;       // 7.5% ticket tax

fn main() {
    let base_fare: f64 = 249.99;

    // Subtotal = fare + baggage fee.
    let subtotal = base_fare + BAGGAGE_FEE_USD;

    // Tax is applied to the subtotal.
    let tax = subtotal * TAX_RATE;

    let total = subtotal + tax;

    println!("Base fare:   ${:.2}", base_fare);
    println!("Baggage fee: ${:.2}", BAGGAGE_FEE_USD);
    println!("Subtotal:    ${:.2}", subtotal);
    println!("Tax ({:.1}%):  ${:.2}", TAX_RATE * 100.0, tax);
    println!("Total:       ${:.2}", total);
}
