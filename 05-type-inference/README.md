# Lesson 05 — Type inference and type annotations

## Concept

You usually do **not** have to write types in Rust — the compiler infers them from the value:

```rust
let passenger_count = 180;        // compiler infers i32
```

But sometimes you must — or want to — be explicit:

```rust
let baggage_weight_kg: f32 = 23.5;  // force an f32 instead of default f64
```

Type annotations are useful when:

- The compiler cannot infer the type from context
- You want a specific numeric size (e.g., `u8`, `i64`)
- You want to make intent obvious to a future reader

## What this program shows

- Some airline values with **inferred** types.
- Other values with **explicit** type annotations.

## Run it

```bash
./run.sh 05-type-inference
```

## Expected output

```
Flight AA1234 to San Francisco
Passenger count (inferred i32): 180
Avg fare USD    (inferred f64): 249.99
Baggage kg      (explicit f32): 23.5
Pilots onboard  (explicit u8):  2
```
