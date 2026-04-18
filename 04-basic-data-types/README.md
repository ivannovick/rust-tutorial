# Lesson 04 — Basic data types

## Concept

Rust is **statically typed** — every value has a type known at compile time. The most common primitive types are:

| Type      | Example            | Meaning                         |
|-----------|--------------------|---------------------------------|
| `i32`     | `42`               | 32-bit signed integer           |
| `u32`     | `42u32`            | 32-bit unsigned integer         |
| `f64`     | `3.14`             | 64-bit floating point           |
| `bool`    | `true` / `false`   | boolean                         |
| `char`    | `'A'`              | single Unicode scalar value     |
| `&str`    | `"Gate 5"`         | string slice (text reference)   |

## What this program shows

- Storing a gate number (`i32`), a ticket price (`f64`), an on-time flag (`bool`), and a boarding zone letter (`char`).
- Printing values of different types with `println!`.

## Run it

```bash
./run.sh 04-basic-data-types
```

## Expected output

```
=== Flight Info ===
Flight:        AA1234
Gate:          17
Ticket price:  $249.99
On time:       true
Boarding zone: B
```
