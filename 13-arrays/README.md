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

- An array of the next 7 departing flight numbers.
- Printing them one by one.

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
```
