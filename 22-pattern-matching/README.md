# Lesson 22 — Pattern matching

## Concept

`match` compares a value against a series of patterns and runs the code for the first match. Every possible value **must** be handled — the compiler enforces this.

```rust
match status {
    FlightStatus::OnTime   => { ... }
    FlightStatus::Delayed  => { ... }
    FlightStatus::Cancelled => { ... }
}
```

Patterns can:

- match specific values (`1 => ...`)
- bind variables (`Some(x) => ...`)
- use ranges (`1..=5 => ...`)
- fall through with a wildcard (`_ => ...`)

`match` is also an **expression**, so you can assign its result to a variable.

## What this program shows

- A `FlightStatus` enum with four variants (one carries data).
- `match` chooses the right staff action for each status.

## Run it

```bash
./run.sh 22-pattern-matching
```

## Expected output

```
Flight AA1234 is OnTime       -> Open gate and begin boarding on schedule.
Flight UA5678 is Delayed(45)  -> Announce delay of 45 minutes and reassure passengers.
Flight DL9012 is Cancelled    -> Notify passengers and open rebooking desk.
Flight WN3456 is Boarding     -> Final call — close doors in 5 minutes.
```
