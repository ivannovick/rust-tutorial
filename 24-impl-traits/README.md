# Lesson 24 — `impl`, methods, and traits

## Why traits?

Imagine the airport display board. It has to print a one-line summary for **different kinds of things**: flights, passengers, baggage. Each type formats its line differently (a flight talks about gates; a bag talks about weight), but the caller — the board — just wants *one* function that handles all of them.

**Without traits** you'd end up writing:

```rust
fn print_flight_summary(f: &Flight)       { ... }
fn print_passenger_summary(p: &Passenger) { ... }
fn print_baggage_summary(b: &Baggage)     { ... }
// ...and a separate loop per collection.
```

**With a trait**, you declare a single contract — "anything printable on the board must have a `summary()` method" — and then write **one** function that accepts any of them:

```rust
fn print_to_board(item: &dyn DisplaySummary) {
    println!("{}", item.summary());
}
```

You can also mix types in a single `Vec<&dyn DisplaySummary>` and iterate once, with the right `summary()` called for each element at runtime.

## Terminology

- **`impl` block**: attaches methods to a type. `&self` = borrow, `&mut self` = mutable borrow.
- **Inherent methods**: declared in `impl Flight { ... }`. Only callable on `Flight` values.
- **Trait**: a set of method signatures any type can implement (think: interface).
- **Trait impl**: `impl DisplaySummary for Flight { ... }` — wires a type to a trait.
- **`&dyn Trait`**: a reference to some value that implements `Trait` — enables heterogeneous collections.

## What this program shows

- Three very different types — `Flight`, `Passenger`, `Baggage` — each implementing the **same** trait `DisplaySummary` in its **own** way (different wording, different fields, even a conditional `*FF*` / `!!HEAVY!!` tag).
- One `print_to_board` function that accepts any `&dyn DisplaySummary`.
- A single `Vec<&dyn DisplaySummary>` holding all three types, iterated in one loop.
- `Flight` also has **inherent** methods (`is_full`, `check_in`) that only exist on `Flight` — showing the difference between trait methods and type-specific methods.

## Run it

```bash
./run.sh 24-impl-traits
```

## Expected output

```
=== Single function, three different types ===
[FLIGHT]    AA1234 -> San Francisco | gate 17 | 2 seats left
[PASSENGER] Alice Nguyen in seat 14A *FF*
[BAGGAGE]   tag BAG-0001 | 18.5 kg

=== Mixed Vec, one loop ===
[FLIGHT]    AA1234 -> San Francisco | gate 17 | 2 seats left
[PASSENGER] Alice Nguyen in seat 14A *FF*
[PASSENGER] Bob Patel in seat 22C
[BAGGAGE]   tag BAG-0001 | 18.5 kg
[BAGGAGE]   tag BAG-0002 | 27.2 kg  !!HEAVY!!

=== Inherent methods only on Flight ===
Flight full? false
Checking in a passenger...
Checking in another passenger...
Flight full now? true
```
