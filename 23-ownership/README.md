# Lesson 23 — Ownership, borrowing, and memory basics

## Concept

Rust has **no garbage collector**. Instead it uses an **ownership** system:

1. Every value has exactly one **owner** at a time.
2. When the owner goes out of scope, the value is dropped (memory freed).
3. You can **move** ownership, or you can **borrow** a reference without taking ownership.

Two reference forms:

| Form  | What it gives you                 |
|-------|-----------------------------------|
| `&T`  | Shared (read-only) borrow         |
| `&mut T` | Exclusive (read+write) borrow  |

Rules of borrowing (checked at compile time):

- You can have **many** `&T` references, OR **one** `&mut T` — never both.
- References must not outlive the data they point to.

## What this program shows

- `take_note` **takes ownership** of a `String` — the caller can't use it again.
- `print_name` **borrows** a `&String` — the caller keeps ownership.

## Run it

```bash
./run.sh 23-ownership
```

## Expected output

```
Printing name (borrowed): Alice Nguyen
Name is still usable here: Alice Nguyen
Recorded booking note: VIP traveler — priority boarding
```
