# Lesson 01 — Hello World and `main`

## Concept

Every Rust program starts executing from a special function named `main`. When you run a Rust binary, the operating system calls `main()` — nothing else runs unless `main` calls it.

`println!` (note the `!` — it is a **macro**, not a regular function) writes a line of text to standard output.

## What this program shows

- The standard shape of a Rust program.
- That `main` is the entry point.
- How to print text with `println!`.

## Run it

From the `rust-tutorial/` directory:

```bash
./run.sh 01-hello-world
```

## Expected output

```
Airline Management System starting...
Execution begins in main()
```
