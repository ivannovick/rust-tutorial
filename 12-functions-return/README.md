# Lesson 12 — Functions with return values

## Concept

A function's return type goes after `->`:

```rust
fn base_fare() -> f64 {
    199.99
}
```

Two important Rust quirks:

1. The **last expression** in a function is the return value — **no semicolon** at the end.
2. You can still use `return value;` early in the function for clarity or early exit.

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b   // no semicolon — this is the returned value
}
```

## What this program shows

- `base_fare` returns a fixed fare.
- `baggage_charge` returns a cost based on the number of bags.
- `total` combines both to return the final ticket cost.

## Run it

```bash
./run.sh 12-functions-return
```

## Expected output

```
Base fare:      $249.99
Baggage (2 bags): $60.00
Total:          $309.99
```
