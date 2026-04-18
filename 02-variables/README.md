# Lesson 02 — Variables with `let`

## Concept

In Rust, you introduce a variable binding with the `let` keyword:

```rust
let passenger_name = "Alice";
```

By default, bindings are **immutable** — once set, they cannot be changed. (You will see `mut` in the next lesson.)

Rust variable names use `snake_case` by convention.

## What this program shows

- Storing airline booking values (passenger name, flight number, destination) in variables.
- Using `{}` placeholders in `println!` to print variable values.

## Run it

```bash
./run.sh 02-variables
```

## Expected output

```
=== Booking Summary ===
Passenger:   Alice Nguyen
Flight:      AA1234
Destination: San Francisco
```
