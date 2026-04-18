// Lesson 25 — Intro to concurrency: threads and channels
//
// `thread::spawn` runs work on a separate OS thread.
// `mpsc::channel` is a multi-producer, single-consumer channel used to
// send messages between threads.
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Create a channel for sending Strings.
    // `tx` is the transmitter (producer), `rx` is the receiver (consumer).
    let (tx, rx) = mpsc::channel::<String>();

    // Spawn a worker thread that simulates an airport operations feed.
    // The closure takes ownership of `tx` via `move`, so it can call
    // `tx.send(...)` from inside the thread.
    let worker = thread::spawn(move || {
        let updates = [
            "Flight AA1234: boarding",
            "Flight UA5678: delayed by 15 minutes",
            "Flight DL9012: cancelled",
            "Flight WN3456: on time",
        ];

        for msg in updates {
            // A small sleep just to simulate time passing between updates.
            thread::sleep(Duration::from_millis(50));
            // `send` returns a Result. In a tiny demo like this it won't
            // fail, so we `unwrap`.
            tx.send(msg.to_string()).unwrap();
        }
        // When the closure ends, `tx` is dropped, which closes the
        // channel. That causes `rx.recv()` to return `Err` next time.
    });

    // Main thread: receive messages until the channel is closed.
    // Iterating over `rx` ends automatically when the sender is dropped.
    for message in rx {
        println!("[airport ops] {}", message);
    }

    // Wait for the worker thread to finish before exiting `main`.
    worker.join().unwrap();
    println!("Control tower shutting down after all updates.");
}
