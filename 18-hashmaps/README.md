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
- Print each route.

## Run it

```bash
./run.sh 18-hashmaps
```

## Expected output

```
AA1234 -> San Francisco
UA5678 -> Chicago
DL9012 -> Atlanta

Looking up AA1234: San Francisco
Updating UA5678 destination to Denver...

AA1234 -> San Francisco
UA5678 -> Denver
DL9012 -> Atlanta
```
