# Lesson 15 — Slices

## Concept

A **slice** is a reference to a contiguous section of an array or vector. It does **not** own the data — it only borrows a view of it:

```rust
let seats = [1, 2, 3, 4, 5, 6];
let first_half = &seats[0..3];   // -> [1, 2, 3]
let rest       = &seats[3..];    // -> [4, 5, 6]
```

Syntax:

- `&arr[start..end]` — elements `start` through `end-1`
- `&arr[start..]`    — from `start` to the end
- `&arr[..end]`      — from the start up to `end-1`

Slice type is written as `&[T]`.

## What this program shows

- A fixed array of seat numbers.
- Two slices: one covering economy seats, one covering business seats.

## Run it

```bash
./run.sh 15-slices
```

## Expected output

```
All seats:       [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
Business seats:  [1, 2, 3]
Economy seats:   [4, 5, 6, 7, 8, 9, 10]
```
