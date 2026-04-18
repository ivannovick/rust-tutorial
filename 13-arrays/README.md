# Lesson 13 — Arrays

## Concept

An **array** in Rust is a fixed-size sequence of values of the same type. Its length is part of its type:

```rust
let flights: [&str; 3] = ["AA1", "UA2", "DL3"];
```

Key properties:

- Length is fixed at compile time.
- All elements share a single type.
- Stored on the stack (no heap allocation).

Access elements with `flights[0]`. Iterate with `for f in flights`.

For resizable sequences, see Lesson 17 (Vectors).

## What this program shows

- An array of the next 7 departing flight numbers (`[&str; 7]`).
- A second array of 20 active gate numbers (`[i32; 20]`) — showing an integer array of a different length.
- Iterating each, plus using `.len()` to ask an array its size.

## Run it

```bash
./run.sh 13-arrays
```

## Expected output

```
=== Next 7 Departures ===
1. AA1234
2. UA5678
3. DL9012
4. WN3456
5. AS7890
6. B61122
7. NK3344

=== 20 Active Gates in Terminal B ===
1 2 3 4 5 7 8 9 11 12 14 15 17 18 19 20 22 24 25 27
(total gates: 20)
```
