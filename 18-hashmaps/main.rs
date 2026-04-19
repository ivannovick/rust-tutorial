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

    // ------------------------------------------------------------------
    println!();
    println!("=== More HashMap conveniences ===");

    // len() / is_empty() — how many entries does the map have?
    println!("Route count: {} (empty? {})", routes.len(), routes.is_empty());

    // contains_key() — membership check without pulling the value out.
    println!("Have AA1234? {}", routes.contains_key("AA1234"));
    println!("Have XX9999? {}", routes.contains_key("XX9999"));

    // remove() — delete a key, returns Option<V> with the old value.
    if let Some(old) = routes.remove("DL9012") {
        println!("Removed DL9012 (was heading to {})", old);
    }

    // HashMap::from() — build a map from an array of (K, V) tuples.
    // Handy for tests and small static tables.
    let mut gate_assignments: HashMap<String, u32> = HashMap::from([
        ("AA1234".to_string(), 17),
        ("UA5678".to_string(), 22),
        ("WN3456".to_string(), 3),
    ]);

    // keys() / values() — iterators over keys or values only.
    let mut gate_nums: Vec<&u32> = gate_assignments.values().collect();
    gate_nums.sort();
    println!("All gate numbers (sorted): {:?}", gate_nums);

    // iter_mut() — mutate values in place. Here we "renumber" all gates
    // by adding 100 to each (e.g. Terminal B uses gates 100+).
    for (_flight, gate) in gate_assignments.iter_mut() {
        *gate += 100;
    }
    let mut updated_gates: Vec<(&String, &u32)> = gate_assignments.iter().collect();
    updated_gates.sort_by(|a, b| a.0.cmp(b.0));
    println!("After renumbering: {:?}", updated_gates);

    // --- Entry API: the single most useful HashMap feature ------------
    //
    // A "bag-per-flight" counter: for each piece of baggage scanned, bump
    // the count for its flight. entry(k).or_insert(0) returns a mutable
    // reference to the value, creating it with the default if it was
    // missing. Then `+= 1` increments it.
    let scans = ["AA1234", "UA5678", "AA1234", "AA1234", "UA5678", "WN3456"];
    let mut bag_counts: HashMap<String, u32> = HashMap::new();

    for flight in scans {
        let entry = bag_counts.entry(flight.to_string()).or_insert(0);
        *entry += 1;
    }

    let mut counts: Vec<(&String, &u32)> = bag_counts.iter().collect();
    counts.sort_by(|a, b| a.0.cmp(b.0));
    println!("Bag counts: {:?}", counts);

    // entry().and_modify().or_insert() — "upsert" in one expression.
    // If UA5678 already exists, add 5; otherwise start at 5.
    bag_counts
        .entry("UA5678".to_string())
        .and_modify(|n| *n += 5)
        .or_insert(5);
    println!("After upsert UA5678 += 5: {:?}", bag_counts.get("UA5678"));

    // extend() — bulk insert from another iterator of (K, V) pairs.
    bag_counts.extend([
        ("DL9012".to_string(), 4),
        ("B61122".to_string(), 7),
    ]);
    println!("After extend, map size: {}", bag_counts.len());

    // clear() — remove every entry at once.
    bag_counts.clear();
    println!("After clear: empty? {}", bag_counts.is_empty());
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
