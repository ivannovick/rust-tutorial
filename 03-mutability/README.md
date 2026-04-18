# Lesson 03 — Mutability with `mut`

## Concept

Rust bindings are immutable by default. To allow a variable to change, add the `mut` keyword:

```rust
let mut seats = 10;
seats = seats - 1;
```

Mutability is opt-in because unchanging values are easier to reason about and less error-prone.

## What this program shows

- A seat-availability counter that starts at a known value.
- The counter is decremented as bookings happen.
- The new value is printed after each update.

## Run it

```bash
./run.sh 03-mutability
```

## Expected output

```
Starting with 10 seats available on flight AA1234
Booking 1 confirmed — seats available: 9
Booking 2 confirmed — seats available: 8
Booking 3 confirmed — seats available: 7
Final seat count: 7
```
