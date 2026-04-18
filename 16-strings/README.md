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
- A tour of commonly used string methods:

| Method                           | What it does                                    |
|----------------------------------|-------------------------------------------------|
| `.len()`                         | length in **bytes**                             |
| `.chars().count()`               | length in **characters** (Unicode scalars)      |
| `.is_empty()`                    | true if length is 0                             |
| `.to_uppercase()` / `.to_lowercase()` | case-converted copy                        |
| `.trim()`                        | strip leading/trailing whitespace               |
| `.contains(sub)`                 | substring search                                |
| `.starts_with(p)` / `.ends_with(s)` | prefix / suffix check                        |
| `.replace(from, to)`             | return a new String with replacements           |
| `.split(delim)`                  | iterator over pieces                            |
| `format!(...)`                   | build a new String like `println!`              |
| `a + &b`                         | concatenate (moves `a`, borrows `b`)            |
| `.parse::<T>()`                  | convert text to another type (returns `Result`) |

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

=== Common String Operations ===
Name length (bytes): 12
Name length (chars): 12
Middle name empty? true
Upper: SAN FRANCISCO, CA
Lower: san francisco, ca
Trimmed flight code: 'AA1234'
Contains '12'?   true
Starts with 'AA'? true
Ends with '34'?  true
Original: Flight AA1234 to San Francisco
Rerouted: Flight AA1234 to San Diego
Manifest: [Alice] [Bob] [Carol] [Dan]
Boarding pass: AA1234 | San Francisco, CA | Alice Nguyen
Welcome, Alice Nguyen
Parsed seat number: 14 (next seat: 15)
```
