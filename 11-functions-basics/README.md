# Lesson 11 — Functions: basics

## Concept

Functions are declared with `fn`:

```rust
fn welcome_banner() {
    println!("Welcome!");
}
```

You **call** a function by writing its name followed by `()`. Parameters and return types, when present, go in the signature:

```rust
fn greet(name: &str) { ... }
```

Rust doesn't care about function order — you can call a function defined further down in the file.

## What this program shows

- Three separate functions: a welcome banner, a flight summary, a boarding message.
- `main` calls each of them in order.

## Run it

```bash
./run.sh 11-functions-basics
```

## Expected output

```
========================================
      Welcome to SkyHigh Airlines
========================================
Flight AA1234 to San Francisco — Gate 17
Boarding now. Please have your pass ready.
```
