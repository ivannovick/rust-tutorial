# Lesson 06 — Constants

## Concept

A `const` is a value fixed at compile time and tied to a name. Unlike `let`:

- You **must** write the type (`const NAME: Type = value;`).
- The value must be a constant expression — no runtime computation.
- Constants can be declared at module scope (outside any function).
- By convention, names are `SCREAMING_SNAKE_CASE`.

```rust
const BAGGAGE_FEE_USD: f64 = 30.0;
const TAX_RATE: f64 = 0.075;
```

Use constants for values that never change: fees, rates, limits, messages.

## What this program shows

- Declaring airline-wide constants at the top of the file.
- Using them in a simple fare calculation.

## Run it

```bash
./run.sh 06-constants
```

## Expected output

```
Base fare:   $249.99
Baggage fee: $30.00
Subtotal:    $279.99
Tax (7.5%):  $21.00
Total:       $300.99
```
