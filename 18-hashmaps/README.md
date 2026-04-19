# Lesson 18 — HashMaps

## Concept

A `HashMap<K, V>` stores key/value pairs. It's great for lookups by name or ID:

```rust
use std::collections::HashMap;

let mut routes: HashMap<String, String> = HashMap::new();
routes.insert("AA1234".to_string(), "San Francisco".to_string());

let dest = routes.get("AA1234");    // returns Option<&String>
```

Key things to know:

- `HashMap` lives in `std::collections`, so you must `use` it.
- `insert` on an existing key **replaces** the value.
- `get` returns `Option<&V>` — `Some(&value)` on hit, `None` on miss.

## What this program shows

- A route map from flight numbers to destinations.
- Insert, update (via `insert` with same key), and look up an entry.
- A tour of commonly used `HashMap` methods:

| Method                                            | What it does                                         |
|---------------------------------------------------|------------------------------------------------------|
| `HashMap::new()` / `HashMap::from([...])`         | empty / from literal array of `(K, V)` tuples        |
| `.insert(k, v)`                                   | add or replace; returns old value as `Option<V>`     |
| `.get(k)`                                         | lookup, returns `Option<&V>`                         |
| `.contains_key(k)`                                | membership check                                     |
| `.remove(k)`                                      | remove key, returns `Option<V>`                      |
| `.len()` / `.is_empty()` / `.clear()`             | size / emptiness / empty everything                  |
| `.keys()` / `.values()` / `.iter()`               | read-only iterators                                  |
| `.values_mut()` / `.iter_mut()`                   | iterate and mutate values in place                   |
| `.entry(k).or_insert(v)`                          | get-or-insert; returns `&mut V` so you can bump it   |
| `.entry(k).and_modify(f).or_insert(v)`            | classic upsert — modify if present, insert if not    |
| `.extend(iter)`                                   | bulk insert from an iterator of `(K, V)` pairs       |

## Run it

```bash
./run.sh 18-hashmaps
```

## Expected output

```
AA1234 -> San Francisco
DL9012 -> Atlanta
UA5678 -> Chicago

Looking up AA1234: San Francisco
Updating UA5678 destination to Denver...

AA1234 -> San Francisco
DL9012 -> Atlanta
UA5678 -> Denver

=== More HashMap conveniences ===
Route count: 3 (empty? false)
Have AA1234? true
Have XX9999? false
Removed DL9012 (was heading to Atlanta)
All gate numbers (sorted): [3, 17, 22]
After renumbering: [("AA1234", 117), ("UA5678", 122), ("WN3456", 103)]
Bag counts: [("AA1234", 3), ("UA5678", 2), ("WN3456", 1)]
After upsert UA5678 += 5: Some(7)
After extend, map size: 5
After clear: empty? true
```
