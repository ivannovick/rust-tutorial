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

    println!();
    println!("=== More Vec conveniences ===");

    // pop() — remove and return the LAST element as Option<T>.
    // Returns None if the vector is empty, so you never panic on an empty pop.
    if let Some(last) = standby_list.pop() {
        println!("Last-minute cancellation: {} left standby", last);
    }

    // insert(index, value) — put a value at an arbitrary position.
    // Existing items at/after `index` shift right by one.
    standby_list.insert(0, String::from("VIP: Zoe Chen"));
    print_standby(&standby_list);

    // contains(&needle) — true if `needle` is present. Note the `&`.
    let looking_for = String::from("Alice Nguyen");
    println!("Alice on list? {}", standby_list.contains(&looking_for));

    // iter().position(|x| cond) — find the FIRST index matching a predicate.
    // Returns Option<usize>.
    match standby_list.iter().position(|name| name == "Alice Nguyen") {
        Some(idx) => println!("Alice is at index {}", idx),
        None      => println!("Alice not found"),
    }

    // first() / last() — safe peek at the ends. Both return Option<&T>.
    println!("First on list: {:?}", standby_list.first());
    println!("Last on list:  {:?}", standby_list.last());

    // get(i) — safe indexed access. Returns None instead of panicking
    // when the index is out of bounds. Compare to `list[i]` which panics.
    println!("Slot 99: {:?}", standby_list.get(99));

    // sort() — in-place alphabetical sort (works because String: Ord).
    standby_list.sort();
    println!("After sort():");
    print_standby(&standby_list);

    // reverse() — in-place reversal.
    standby_list.reverse();
    println!("After reverse():");
    print_standby(&standby_list);

    // --- Numeric vectors ---------------------------------------------
    // `vec![...]` is a macro that builds a Vec from a list of values.
    let bag_weights_kg: Vec<i32> = vec![12, 18, 7, 23, 30, 15, 22];
    println!();
    println!("Bag weights (kg): {:?}", bag_weights_kg);

    // iter().sum() — sum of all elements. Type annotation needed because
    // `sum` is generic over the output type.
    let total: i32 = bag_weights_kg.iter().sum();
    println!("Total weight: {} kg", total);

    // iter().max() / .min() — largest/smallest. Return Option because
    // an empty vector has neither.
    println!("Heaviest bag: {:?} kg", bag_weights_kg.iter().max());
    println!("Lightest bag: {:?} kg", bag_weights_kg.iter().min());

    // filter() + collect() — keep only items matching a predicate.
    // The turbofish `::<Vec<_>>` tells collect what collection to build.
    let overweight: Vec<i32> = bag_weights_kg
        .iter()
        .copied()             // `iter()` yields &i32; copied() turns it into i32
        .filter(|w| *w > 20)  // predicate sees &i32; deref with *
        .collect();
    println!("Overweight (> 20 kg): {:?}", overweight);

    // map() + collect() — transform every element.
    let weights_lb: Vec<i32> = bag_weights_kg
        .iter()
        .map(|kg| (*kg as f64 * 2.2) as i32) // rough kg -> lb
        .collect();
    println!("Weights in lb:   {:?}", weights_lb);

    // extend() — append another iterator onto a vec.
    let mut all_bags = bag_weights_kg.clone();
    all_bags.extend(vec![9, 14]);
    println!("After extend:    {:?}", all_bags);

    // clear() — empty the vec. is_empty() lets you verify.
    all_bags.clear();
    println!("After clear: empty? {}, len = {}", all_bags.is_empty(), all_bags.len());
}

// Borrows the vector so the caller keeps ownership.
fn print_standby(list: &Vec<String>) {
    println!("Standby list size: {}", list.len());
    for (i, name) in list.iter().enumerate() {
        println!("  {}. {}", i + 1, name);
    }
}
