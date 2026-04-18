# Lesson 10 — `while` and `for`

## Concept

- **`while`** loops run as long as a condition is `true`. Best for: "keep going until some condition changes".
- **`for`** loops iterate over a sequence (range, array, vector, etc.). Best for: "do this once per item".

```rust
while gate <= 5 { ... }
for flight in ["AA1", "UA2", "DL3"] { ... }
```

Ranges use `..` (exclusive end) or `..=` (inclusive end):

- `0..5`  produces `0, 1, 2, 3, 4`
- `0..=5` produces `0, 1, 2, 3, 4, 5`

## What this program shows

- A `while` loop prints gate numbers 1 through 5.
- A `for` loop prints a short list of flight numbers.

## Run it

```bash
./run.sh 10-while-for
```

## Expected output

```
=== Gates (while) ===
Gate 1 is ready
Gate 2 is ready
Gate 3 is ready
Gate 4 is ready
Gate 5 is ready

=== Flights (for) ===
Upcoming flight: AA1234
Upcoming flight: UA5678
Upcoming flight: DL9012
Upcoming flight: WN3456
```
