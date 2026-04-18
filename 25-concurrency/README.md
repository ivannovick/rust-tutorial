# Lesson 25 — Intro to concurrency: threads and channels

## Concept

Rust has first-class support for threads in its standard library:

```rust
use std::thread;

let handle = thread::spawn(|| {
    println!("hi from another thread");
});

handle.join().unwrap();
```

- `thread::spawn` starts a new OS thread running the given closure.
- `join()` waits for the thread to finish.

To send data **between** threads, use a **channel**:

```rust
use std::sync::mpsc;

let (tx, rx) = mpsc::channel::<String>();
tx.send("boarding".to_string()).unwrap();
let msg = rx.recv().unwrap();
```

`mpsc` = "multiple producer, single consumer."
`tx` is the sender (can be cloned), `rx` is the receiver.

## What this program shows

- A background thread simulates flight updates.
- It sends messages like `"boarding"` and `"delayed"` through a channel.
- The main thread receives and prints them, then joins the worker.

## Run it

```bash
./run.sh 25-concurrency
```

## Expected output

```
[airport ops] Flight AA1234: boarding
[airport ops] Flight UA5678: delayed by 15 minutes
[airport ops] Flight DL9012: cancelled
[airport ops] Flight WN3456: on time
Control tower shutting down after all updates.
```
