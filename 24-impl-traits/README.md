# Lesson 24 — `impl`, methods, and traits

## Concept

You attach **methods** to a type using an `impl` block:

```rust
impl Flight {
    fn is_full(&self) -> bool {
        self.seats_available == 0
    }
}
```

`&self` means "a shared reference to the instance I'm being called on."
`&mut self` means "I need to modify the instance."

A **trait** is a set of methods a type can implement — similar to an interface in other languages:

```rust
trait DisplaySummary {
    fn summary(&self) -> String;
}
```

Multiple types can implement the same trait, enabling polymorphism.

## What this program shows

- A `Flight` struct with methods `is_full()` and `check_in()`.
- A `DisplaySummary` trait implemented by both `Flight` and `Passenger`.
- A single loop prints summaries of mixed types by calling `.summary()`.

## Run it

```bash
./run.sh 24-impl-traits
```

## Expected output

```
[summary] Flight AA1234 to San Francisco - Gate 17 - 2 seats left
[summary] Passenger Alice Nguyen (Confirmed)
Flight AA1234 full? false
Checking in a passenger on AA1234...
Flight AA1234 full? false
Checking in a passenger on AA1234...
Flight AA1234 full? true
```
