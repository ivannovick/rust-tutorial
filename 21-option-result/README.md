# Lesson 21 — `Option` and `Result`

## Concept

Rust does not have `null`. It replaces it with two enum types from the standard library:

```rust
enum Option<T> {
    Some(T),    // a value is present
    None,       // no value
}

enum Result<T, E> {
    Ok(T),      // success — value of type T
    Err(E),     // failure — error of type E
}
```

Use them to make "might fail" and "might be missing" visible at the type level:

- `Option<T>` — the value might be absent.
- `Result<T, E>` — the operation might fail with an error.

You inspect them with `match`, or with helpers like `.unwrap_or(...)`.

## What this program shows

- `find_seat` returns `Option<u32>` — Some(seat) or None.
- `parse_weight` returns `Result<f64, ...>` — Ok(weight) or Err(parse error).

## Run it

```bash
./run.sh 21-option-result
```

## Expected output

```
Searching for passenger 'Alice'...
  Seat found: 14
Searching for passenger 'Zoe'...
  No seat assigned.

Parsing baggage weight '23.5'...
  Weight: 23.50 kg
Parsing baggage weight 'heavy'...
  Parse error: invalid float literal
```
