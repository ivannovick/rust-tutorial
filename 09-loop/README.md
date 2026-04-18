# Lesson 09 — `loop`

## Concept

`loop` is Rust's infinite loop. You exit it with `break`:

```rust
loop {
    // ... do something ...
    if done { break; }
}
```

`loop` can also **return a value** from the `break` statement:

```rust
let result = loop { break 42; };  // result == 42
```

Use `loop` when you don't know in advance how many iterations you need.

## What this program shows

- A boarding countdown that starts at a number and decrements each step.
- Uses `break` to exit the loop when the countdown reaches zero.

## Run it

```bash
./run.sh 09-loop
```

## Expected output

```
Final boarding call for flight AA1234
Boarding in 5 minutes...
Boarding in 4 minutes...
Boarding in 3 minutes...
Boarding in 2 minutes...
Boarding in 1 minute...
Now boarding!
```
