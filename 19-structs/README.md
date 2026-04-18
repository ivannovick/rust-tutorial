# Lesson 19 — Structs

## Concept

A `struct` groups related fields into a named type:

```rust
struct Flight {
    number: String,
    destination: String,
    gate: u32,
    seats_available: u32,
}
```

Create an instance with field/value pairs:

```rust
let f = Flight {
    number: String::from("AA1234"),
    destination: String::from("San Francisco"),
    gate: 17,
    seats_available: 42,
};
```

Access fields with `.`:  `f.destination`.

Structs are the main way to model domain objects in Rust.

## What this program shows

- A `Flight` struct with four fields.
- Three flight instances.
- A simple print function that takes a `&Flight` and shows all fields.

## Run it

```bash
./run.sh 19-structs
```

## Expected output

```
Flight AA1234 -> San Francisco (Gate 17, 42 seats)
Flight UA5678 -> Chicago       (Gate 22, 8 seats)
Flight DL9012 -> Atlanta       (Gate 3, 110 seats)
```
