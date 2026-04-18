// Lesson 17 — Vectors
//
// `Vec<T>` is a growable, heap-allocated array. Use it when you don't know
// the size ahead of time or when the collection changes over time.
fn main() {
    // Start with an empty vector of Strings.
    let mut standby_list: Vec<String> = Vec::new();

    // Add three passengers. `push` appends to the end.
    standby_list.push(String::from("Alice Nguyen"));
    standby_list.push(String::from("Bob Patel"));
    standby_list.push(String::from("Carol Singh"));

    print_standby(&standby_list);

    println!();
    println!("Removing Bob Patel from standby...");
    println!();

    // Remove by index. `remove(1)` removes the SECOND item (Bob).
    // Items after it shift left by one position.
    standby_list.remove(1);

    print_standby(&standby_list);
}

// Borrows the vector so the caller keeps ownership.
fn print_standby(list: &Vec<String>) {
    println!("Standby list size: {}", list.len());
    for (i, name) in list.iter().enumerate() {
        println!("  {}. {}", i + 1, name);
    }
}
