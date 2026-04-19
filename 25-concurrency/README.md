# Lesson 25 — Concurrency: shared work queue with a Mutex

## Scenario

A flight-booking system. Two kinds of threads cooperate through a shared FIFO queue:

- **3 order threads** — each books 20 flights. Between bookings the thread sleeps a random 1–5 seconds (simulating customer think-time), then pushes an `Order` onto the queue.
- **1 email thread** — pops orders off the queue and "sends" a confirmation by printing (stand-in for a real SMTP call).

When all 3 order threads finish, `main` signals the email thread to stop; it first drains any remaining orders, then exits. `main` asserts the queue is empty at the end.

## Do we actually need a lock?

**Yes — whenever two or more threads touch the same mutable data.** Without synchronization you get torn writes, lost updates, and (for linked data structures like a queue) outright memory corruption.

**What makes Rust different:**

In C/C++ forgetting the lock is a silent, intermittent bug. In Rust it is a **compile error**. The `Send`/`Sync` traits plus the borrow checker refuse to let you share mutable state between threads without some form of synchronization.

To share mutable state across threads you must pick one of:

| Tool              | Good for                                               |
|-------------------|--------------------------------------------------------|
| `Arc<Mutex<T>>`   | Shared ownership + one-writer-at-a-time **(used here)** |
| `Arc<RwLock<T>>`  | Many readers OR one writer                             |
| `mpsc` channels   | Message passing — no shared memory                     |
| Atomic types      | Lock-free counters/flags (we also use `AtomicU32` + `AtomicBool`) |

The takeaway: **you cannot write a data race in safe Rust.** The classic "forgot to lock" bug doesn't compile.

## What this program shows

- `Arc<Mutex<VecDeque<Order>>>` as the canonical shared-FIFO-queue pattern.
- Holding the lock for the *minimum* time (pop under lock, print after releasing).
- `AtomicU32::fetch_add` for a lock-free ID counter.
- `AtomicBool` as a shutdown flag.
- A clean shutdown protocol: join producers → set flag → let consumer drain → join consumer → assert empty.

## Run it

```bash
./run.sh 25-concurrency
```

**Heads-up:** the simulation uses real 1–5 second sleeps × 20 iterations per thread. Expect the program to run for roughly **60 seconds of wall-clock time**. If you'd rather watch it fly by, change `Duration::from_secs(sleep_s)` to `Duration::from_millis(sleep_s * 50)` in the source and rebuild.

## Expected output (excerpt — real run is 60+ interleaved lines)

Order and email lines interleave in non-deterministic order. You'll see things like:

```
[email] ready
[thread 1] booked order #1 (slept 3s), queue len = 1
[email] sent confirmation -> passenger Passenger-1-01, flight AA1100001, order #1
[thread 2] booked order #2 (slept 4s), queue len = 1
[email] sent confirmation -> passenger Passenger-2-01, flight AA1200001, order #2
[thread 3] booked order #3 (slept 5s), queue len = 1
...
[thread 3] done — 20 orders submitted
[thread 1] done — 20 orders submitted
[thread 2] done — 20 orders submitted
[main] all 3 order threads finished
[email] sent confirmation -> passenger Passenger-2-20, flight AA1200020, order #60
[email] shutting down — queue drained
[main] final queue length: 0 (expected 0)
[main] all done — 60 orders booked and confirmed
```

60 orders are booked in total (3 threads × 20 each). The email thread never misses one, and the queue ends at length 0 — verified by the `assert_eq!` in `main`.
