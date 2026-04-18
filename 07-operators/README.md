# Lesson 07 — Arithmetic and comparison operators

## Concept

Rust has the usual operators you would expect:

| Category     | Operators                     |
|--------------|-------------------------------|
| Arithmetic   | `+`  `-`  `*`  `/`  `%`       |
| Comparison   | `==` `!=` `<` `>` `<=` `>=`   |
| Logical      | `&&` `\|\|` `!`                 |

Comparison operators return a `bool` (`true` or `false`).

## What this program shows

- Adding base fare + baggage fee to get a total.
- Comparing the total to a discount threshold.
- Printing whether a promo code applies.

## Run it

```bash
./run.sh 07-operators
```

## Expected output

```
Base fare:     $249.99
Baggage fee:   $30.00
Total:         $279.99
Threshold:     $300.00
Qualifies for promo? false
```
