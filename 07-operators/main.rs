// Lesson 07 — Arithmetic and comparison operators
//
// Arithmetic: + - * / %   Comparison: == != < > <= >=
// Comparison expressions produce a `bool`.
fn main() {
    let base_fare: f64 = 249.99;
    let baggage_fee: f64 = 30.00;
    let promo_threshold: f64 = 300.00;

    // Arithmetic: sum up the charges.
    let total = base_fare + baggage_fee;

    // Comparison: `>=` yields a bool.
    // A customer gets a promo when their total reaches the threshold.
    let qualifies_for_promo: bool = total >= promo_threshold;

    println!("Base fare:     ${:.2}", base_fare);
    println!("Baggage fee:   ${:.2}", baggage_fee);
    println!("Total:         ${:.2}", total);
    println!("Threshold:     ${:.2}", promo_threshold);
    println!("Qualifies for promo? {}", qualifies_for_promo);
}
