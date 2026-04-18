// Lesson 18 — HashMaps
//
// A HashMap<K, V> stores key/value pairs. Lookups, inserts, and updates
// are all O(1) average. Perfect for flight -> destination routing.
use std::collections::HashMap;

fn main() {
    // Key type: String (flight number). Value type: String (destination).
    let mut routes: HashMap<String, String> = HashMap::new();

    // Insert three flights and their destinations.
    routes.insert("AA1234".to_string(), "San Francisco".to_string());
    routes.insert("UA5678".to_string(), "Chicago".to_string());
    routes.insert("DL9012".to_string(), "Atlanta".to_string());

    print_routes(&routes);

    // Look up a flight. `get` returns Option<&String>.
    // `match` on it to handle both hit and miss cases.
    println!();
    match routes.get("AA1234") {
        Some(destination) => println!("Looking up AA1234: {}", destination),
        None              => println!("Looking up AA1234: not found"),
    }

    // Update: `insert` with an existing key replaces the value.
    println!("Updating UA5678 destination to Denver...");
    routes.insert("UA5678".to_string(), "Denver".to_string());

    println!();
    print_routes(&routes);
}

// Print every (flight -> destination) pair.
// Note: iteration order in a HashMap is NOT guaranteed.
fn print_routes(routes: &HashMap<String, String>) {
    // Collect keys into a Vec and sort them so the output is deterministic.
    let mut keys: Vec<&String> = routes.keys().collect();
    keys.sort();

    for flight in keys {
        let destination = routes.get(flight).unwrap();
        println!("{} -> {}", flight, destination);
    }
}
