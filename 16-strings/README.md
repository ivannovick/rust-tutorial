# Lesson 16 — Strings: `&str` and `String`

## Concept

Rust has **two** main string types:

| Type     | Owned? | Mutable? | Typical use                               |
|----------|--------|----------|-------------------------------------------|
| `&str`   | no     | no       | Fixed text, parameters, borrowed views    |
| `String` | yes    | yes      | Text you build up or own at runtime       |

- Literal `"hello"` has type `&'static str`.
- `String::from("hello")` or `"hello".to_string()` creates an owned `String`.
- You can append to a `String` with `push_str`.
- You can always borrow a `String` **as** an `&str` (an `&String` coerces to `&str`).

## What this program shows

- Fixed labels are `&str` literals.
- Passenger name and destination are owned `String` values built at runtime.

## Run it

```bash
./run.sh 16-strings
```

## Expected output

```
=== Passenger Record ===
PASSENGER NAME : Alice Nguyen
DESTINATION    : San Francisco, CA
NOTE           : VIP traveler — priority boarding
```
