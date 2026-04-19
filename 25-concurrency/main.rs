// Lesson 25 — Concurrency: shared work queue with a Mutex
//
// Scenario: a flight-booking system.
//
//   - 3 ORDER threads each book 20 flights. Between bookings each one
//     sleeps a random 1–5 seconds (simulating customers).
//     Every booking becomes an Order pushed onto a shared FIFO queue.
//
//   - 1 EMAIL thread pops orders off the queue and "sends" a
//     confirmation by printing a line (stand-in for a real SMTP call).
//
//   - When all 3 order threads finish, `main` signals the email thread
//     to stop. The email thread drains any leftover orders first, then
//     exits. `main` asserts the queue is empty at the end.
//
// =====================================================================
// Do we actually need a lock?
// =====================================================================
// YES — any time two or more threads touch the SAME mutable data, you
// need synchronization to prevent torn writes, lost updates, and invalid
// pointers. In C/C++ forgetting the lock is a silent, intermittent bug.
//
// In Rust the compiler refuses to let you do it. Sharing `&mut T`
// between threads is a compile error — the `Send`/`Sync` traits and
// the borrow checker enforce it. To share mutable state across threads
// you MUST pick one of:
//
//     Arc<Mutex<T>>   — shared ownership + one-writer-at-a-time (used here)
//     Arc<RwLock<T>>  — many readers OR one writer
//     mpsc channels   — send messages instead of sharing memory
//     Atomic types    — lock-free counters/flags (we use AtomicU32 + AtomicBool below)
//
// The payoff: you cannot write a data race in safe Rust. The classic
// "forgot to lock" bug doesn't compile.
// =====================================================================

use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// Items placed on the shared queue.
#[derive(Debug)]
struct Order {
    id: u32,
    passenger: String,
    flight: String,
}

// Tiny pseudo-random in the closed range [1, 5] — good enough for
// simulating a random sleep, no external `rand` crate required.
fn random_1_to_5_secs(thread_id: u32, iter: u32) -> u64 {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos() as u64;
    (nanos
        .wrapping_add((thread_id as u64).wrapping_mul(99_991))
        .wrapping_add((iter as u64).wrapping_mul(131))
        % 5) + 1
}

fn main() {
    // ---- Shared state ------------------------------------------------
    // Arc   => multiple threads can OWN the same allocation.
    // Mutex => only ONE thread can access the inner VecDeque at a time.
    // VecDeque is our FIFO queue: push_back to enqueue, pop_front to dequeue.
    let queue: Arc<Mutex<VecDeque<Order>>> = Arc::new(Mutex::new(VecDeque::new()));

    // Atomic counter for unique order IDs. A counter doesn't need a
    // Mutex — `fetch_add` is an atomic CPU instruction.
    let next_order_id: Arc<AtomicU32> = Arc::new(AtomicU32::new(0));

    // Shutdown signal for the email thread. Also an atomic — it's just
    // a flag, no Mutex needed.
    let shutdown: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

    // ---- Email confirmation thread -----------------------------------
    let email_queue = Arc::clone(&queue);
    let email_shutdown = Arc::clone(&shutdown);
    let email_thread = thread::spawn(move || {
        println!("[email] ready");
        loop {
            // Take one order under the lock, then DROP the lock before
            // "sending." Holding the lock across I/O would needlessly
            // block the producers.
            let next = {
                let mut q = email_queue.lock().unwrap();
                q.pop_front()
                // MutexGuard goes out of scope here -> lock released.
            };

            match next {
                Some(order) => {
                    // Stand-in for a real email send.
                    println!(
                        "[email] sent confirmation -> passenger {}, flight {}, order #{}",
                        order.passenger, order.flight, order.id
                    );
                }
                None => {
                    // Queue is empty. If we've been told to stop, exit.
                    // Otherwise back off briefly and poll again.
                    if email_shutdown.load(Ordering::Acquire) {
                        break;
                    }
                    thread::sleep(Duration::from_millis(100));
                }
            }
        }
        println!("[email] shutting down — queue drained");
    });

    // ---- 3 order-booking threads, each books 20 orders ---------------
    let mut order_threads: Vec<thread::JoinHandle<()>> = Vec::new();
    for thread_id in 1..=3u32 {
        let q = Arc::clone(&queue);
        let id_counter = Arc::clone(&next_order_id);

        let handle = thread::spawn(move || {
            for i in 0..20u32 {
                // Simulated customer think-time: 1–5 seconds.
                let sleep_s = random_1_to_5_secs(thread_id, i);
                thread::sleep(Duration::from_secs(sleep_s));

                // Build the order — nothing shared yet.
                let id = id_counter.fetch_add(1, Ordering::Relaxed) + 1;
                let order = Order {
                    id,
                    passenger: format!("Passenger-{}-{:02}", thread_id, i + 1),
                    flight: format!("AA{}{:03}", 1000 + thread_id * 100, i + 1),
                };

                // Push under the lock. Grab the queue length for logging
                // WHILE we still hold the lock, then release.
                let queue_len = {
                    let mut q_guard = q.lock().unwrap();
                    q_guard.push_back(order);
                    q_guard.len()
                };

                println!(
                    "[thread {}] booked order #{} (slept {}s), queue len = {}",
                    thread_id, id, sleep_s, queue_len
                );
            }
            println!("[thread {}] done — 20 orders submitted", thread_id);
        });
        order_threads.push(handle);
    }

    // ---- Wait for all order threads, then shut down the emailer ------
    for h in order_threads {
        h.join().unwrap();
    }
    println!("[main] all 3 order threads finished");

    // No one will push anymore. Tell the email thread it may exit once
    // the queue is empty.
    shutdown.store(true, Ordering::Release);

    // Wait for the email thread to drain the queue and exit.
    email_thread.join().unwrap();

    // Verify: queue MUST be empty. If this ever fires, there's a bug
    // in our shutdown protocol.
    let final_len = queue.lock().unwrap().len();
    println!("[main] final queue length: {} (expected 0)", final_len);
    assert_eq!(final_len, 0, "queue should be empty after shutdown");

    println!("[main] all done — 60 orders booked and confirmed");
}
