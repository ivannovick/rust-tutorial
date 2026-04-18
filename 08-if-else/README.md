# Lesson 08 — `if` and `else`

## Concept

Rust `if` works like most languages, but with two quirks worth knowing:

1. Conditions must be a `bool` — no implicit "truthiness".
2. `if` is an **expression** — it produces a value, so you can assign its result to a variable.

```rust
let msg = if on_time { "Boarding" } else { "Delayed" };
```

You can chain `else if` branches for multi-way decisions.

## What this program shows

- A flight-status checker that prints a different message depending on the status string.

## Run it

```bash
./run.sh 08-if-else
```

## Expected output

```
Flight AA1234 status: on_time
  --> On time. Boarding will begin shortly.
Flight UA5678 status: delayed
  --> Delayed. Please check the departure board for updates.
Flight DL9012 status: cancelled
  --> Cancelled. Please contact the rebooking desk.
```
