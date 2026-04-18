# Lesson 14 — Tuples

## Concept

A **tuple** groups a fixed number of values — possibly of different types — into one compound value:

```rust
let flight: (&str, &str, &str) = ("AA1234", "San Francisco", "10:30");
```

Two ways to read values out:

```rust
let (num, dest, time) = flight;   // destructuring
let num = flight.0;               // indexing by position
```

Tuples are great as lightweight return values when a function needs to hand back several related pieces of data without defining a `struct`.

## What this program shows

- A function returns a tuple of `(flight_number, destination, departure_time)`.
- The caller destructures the tuple and prints each field.

## Run it

```bash
./run.sh 14-tuples
```

## Expected output

```
Flight AA1234 -> San Francisco at 10:30
```
