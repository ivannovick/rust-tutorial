# Rust Tutorials: Airline Management System

A 25-lesson hands-on introduction to the Rust programming language. Every lesson is built around a small airline management system, so you learn Rust concepts while the examples keep growing into a realistic domain.

## Requirements

All you need is **Docker**. No local Rust toolchain required.

The `rust:latest` image contains the full Rust toolchain (rustc, cargo) and is used for every lesson.

## Directory layout

Each lesson lives in its own folder, numbered `01` through `25`:

```
rust-tutorial/
├── 01-hello-world/
│   ├── README.md
│   └── main.rs
├── 02-variables/
│   ├── README.md
│   └── main.rs
├── ...
└── run.sh       # helper to compile + run any lesson in docker
```

Every lesson folder contains:

- **README.md** — explains the concept and what the program demonstrates
- **main.rs** — a small, heavily commented Rust program

## How to run a lesson

From the `rust-tutorial/` directory, use the helper script:

```bash
./run.sh 01-hello-world
```

This mounts the lesson folder into the rust docker container, compiles `main.rs` with `rustc`, and runs the resulting binary.

### Manual run (without the script)

```bash
cd 01-hello-world
docker run --rm -v "$PWD":/app -w /app rust:latest \
    sh -c "rustc main.rs -o /tmp/program && /tmp/program"
```

On Windows PowerShell:

```powershell
cd 01-hello-world
docker run --rm -v ${PWD}:/app -w /app rust:latest `
    sh -c "rustc main.rs -o /tmp/program && /tmp/program"
```

## Curriculum

| #  | Topic                                    |
|----|------------------------------------------|
| 01 | Hello World and `main`                   |
| 02 | Variables with `let`                     |
| 03 | Mutability with `mut`                    |
| 04 | Basic data types                         |
| 05 | Type inference and type annotations      |
| 06 | Constants                                |
| 07 | Arithmetic and comparison operators      |
| 08 | `if` and `else`                          |
| 09 | `loop`                                   |
| 10 | `while` and `for`                        |
| 11 | Functions: basics                        |
| 12 | Functions with return values             |
| 13 | Arrays                                   |
| 14 | Tuples                                   |
| 15 | Slices                                   |
| 16 | Strings: `&str` and `String`             |
| 17 | Vectors                                  |
| 18 | HashMaps                                 |
| 19 | Structs                                  |
| 20 | Enums                                    |
| 21 | `Option` and `Result`                    |
| 22 | Pattern matching                         |
| 23 | Ownership, borrowing, and memory basics  |
| 24 | `impl`, methods, and traits              |
| 25 | Intro to concurrency: threads and channels |

## Recurring entities

As you progress, the same concepts reappear: `Passenger`, `Flight`, `Booking`, `Gate`, `Baggage`. Earlier lessons use simple values, later lessons build full structs and enums around them.
