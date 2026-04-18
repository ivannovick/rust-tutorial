// Lesson 05 — Type inference and type annotations
//
// Rust can almost always figure out the type for you, but you can also
// annotate a variable explicitly using the `name: Type` syntax.
fn main() {
    // Inferred types. The compiler picks sensible defaults:
    //   integer literals  -> i32
    //   floating literals -> f64
    let flight_number = "AA1234";
    let destination = "San Francisco";
    let passenger_count = 180;          // inferred as i32
    let avg_fare_usd = 249.99;          // inferred as f64

    // Explicit types. Useful when you want a specific size.
    let baggage_weight_kg: f32 = 23.5;  // force single-precision float
    let pilots_onboard: u8 = 2;         // unsigned 8-bit — fits 0..=255

    println!("Flight {} to {}", flight_number, destination);
    println!("Passenger count (inferred i32): {}", passenger_count);
    println!("Avg fare USD    (inferred f64): {}", avg_fare_usd);
    println!("Baggage kg      (explicit f32): {}", baggage_weight_kg);
    println!("Pilots onboard  (explicit u8):  {}", pilots_onboard);
}
