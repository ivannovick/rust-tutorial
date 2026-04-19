# Lesson 17 — Vectors

## Concept

A `Vec<T>` is a **growable** array. Unlike `[T; N]`, its length can change at runtime:

```rust
let mut names: Vec<String> = Vec::new();
names.push(String::from("Alice"));
names.push(String::from("Bob"));
names.remove(0);    // removes the first element
```

`vec![]` is a convenience macro to build one from a list:

```rust
let xs = vec![1, 2, 3];
```

A vector lives on the heap. Iteration works just like arrays: `for name in &names`.

## What this program shows

- An empty standby list created with `Vec::new()`.
- Three passengers added with `push`.
- One passenger removed with `remove`.
- A tour of commonly used `Vec` methods:

| Method                           | What it does                                        |
|----------------------------------|-----------------------------------------------------|
| `Vec::new()` / `vec![...]`       | create empty / from a literal list                  |
| `.push(v)` / `.pop()`            | append / remove last (returns `Option<T>`)          |
| `.insert(i, v)` / `.remove(i)`   | insert at / remove at index (shifts others)         |
| `.len()` / `.is_empty()`         | size / emptiness check                              |
| `.first()` / `.last()`           | safe peek at ends, returns `Option<&T>`             |
| `.get(i)`                        | safe indexed access (`Option<&T>`, no panic)        |
| `.contains(&v)`                  | membership check                                    |
| `.iter().position(pred)`         | find index of first match                           |
| `.sort()` / `.reverse()`         | in-place order changes                              |
| `.clear()`                       | remove all elements                                 |
| `.extend(iter)`                  | append every item from an iterator                  |
| `.iter().sum() / .max() / .min()`| aggregate numeric vecs                              |
| `.iter().filter(pred).collect()` | keep matching items                                 |
| `.iter().map(f).collect()`       | transform every item                                |

## Run it

```bash
./run.sh 17-vectors
```

## Expected output

```
Standby list size: 3
  1. Alice Nguyen
  2. Bob Patel
  3. Carol Singh

Removing Bob Patel from standby...

Standby list size: 2
  1. Alice Nguyen
  2. Carol Singh

=== More Vec conveniences ===
Last-minute cancellation: Carol Singh left standby
Standby list size: 2
  1. VIP: Zoe Chen
  2. Alice Nguyen
Alice on list? true
Alice is at index 1
First on list: Some("VIP: Zoe Chen")
Last on list:  Some("Alice Nguyen")
Slot 99: None
After sort():
Standby list size: 2
  1. Alice Nguyen
  2. VIP: Zoe Chen
After reverse():
Standby list size: 2
  1. VIP: Zoe Chen
  2. Alice Nguyen

Bag weights (kg): [12, 18, 7, 23, 30, 15, 22]
Total weight: 127 kg
Heaviest bag: Some(30) kg
Lightest bag: Some(7) kg
Overweight (> 20 kg): [23, 30, 22]
Weights in lb:   [26, 39, 15, 50, 66, 33, 48]
After extend:    [12, 18, 7, 23, 30, 15, 22, 9, 14]
After clear: empty? true, len = 0
```
