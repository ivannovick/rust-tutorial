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
- The updated list is printed.

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
```
