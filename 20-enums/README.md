# Lesson 20 — Enums

## Concept

An `enum` is a type whose value is one of several named **variants**:

```rust
enum BookingStatus {
    Confirmed,
    Standby,
    CheckedIn,
    Boarded,
}
```

Enums are the right tool when a value must be exactly one of a small set of possibilities. They are compact, type-safe, and pair perfectly with `match` (Lesson 22).

Rust enum variants can also carry data:

```rust
enum Event {
    GateChange(u32),
    Delay { minutes: u32 },
}
```

## What this program shows

- A `BookingStatus` enum with four variants.
- Several passengers with their statuses.
- A helper function that prints a human-readable status label.

## Run it

```bash
./run.sh 20-enums
```

## Expected output

```
Alice Nguyen:  CheckedIn
Bob Patel:     Confirmed
Carol Singh:   Standby
Dan O'Reilly:  Boarded
```
