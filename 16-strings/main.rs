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

    println!();
    println!("=== Common String Operations ===");

    // len() returns the number of BYTES (not characters).
    // chars().count() returns the number of Unicode characters.
    println!("Name length (bytes): {}", passenger_name.len());
    println!("Name length (chars): {}", passenger_name.chars().count());

    // is_empty() — handy check before using a field.
    let empty_middle_name = String::new();
    println!("Middle name empty? {}", empty_middle_name.is_empty());

    // Case conversion — produces a NEW String, does not mutate.
    println!("Upper: {}", destination.to_uppercase());
    println!("Lower: {}", destination.to_lowercase());

    // trim() removes leading/trailing whitespace, returning an &str view.
    let messy_input = "   AA1234   ";
    println!("Trimmed flight code: '{}'", messy_input.trim());

    // Substring search + prefix/suffix checks.
    let flight_code = "AA1234";
    println!("Contains '12'?   {}", flight_code.contains("12"));
    println!("Starts with 'AA'? {}", flight_code.starts_with("AA"));
    println!("Ends with '34'?  {}", flight_code.ends_with("34"));

    // replace() — returns a new String with all matches replaced.
    let original = String::from("Flight AA1234 to San Francisco");
    let rerouted = original.replace("San Francisco", "San Diego");
    println!("Original: {}", original);
    println!("Rerouted: {}", rerouted);

    // split() — iterate over pieces separated by a delimiter.
    let manifest = "Alice,Bob,Carol,Dan";
    print!("Manifest:");
    for name in manifest.split(',') {
        print!(" [{}]", name);
    }
    println!();

    // format!() — build a new String just like println!, but return it.
    let boarding_pass: String = format!(
        "{} | {} | {}",
        flight_code, destination, passenger_name
    );
    println!("Boarding pass: {}", boarding_pass);

    // Concatenation with `+` takes the LHS by value (moves it) and the
    // RHS as `&str`. Use &clone_of_string or format!() if you want to
    // keep both originals.
    let greeting = String::from("Welcome, ");
    let full_greeting = greeting + &passenger_name; // `greeting` is moved here
    println!("{}", full_greeting);

    // parse() — convert text to another type; returns a Result.
    let seat_text = "14";
    let seat_number: i32 = seat_text.parse().unwrap();
    println!("Parsed seat number: {} (next seat: {})", seat_number, seat_number + 1);
}
