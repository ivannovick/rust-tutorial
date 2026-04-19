// Lesson 24 — impl, methods, and traits
//
// Why traits?
// -----------
// Suppose the airport's display board has to print a one-line summary for
// anything that comes in: a Flight, a Passenger, a piece of Baggage. Each
// type formats its line differently, but callers don't care — they just
// want ONE function that prints any of them.
//
// Without traits, you'd end up with:
//     fn print_flight_summary(f: &Flight) { ... }
//     fn print_passenger_summary(p: &Passenger) { ... }
//     fn print_baggage_summary(b: &Baggage) { ... }
//     // and a custom loop per collection...
//
// A trait lets us declare "any type that wants to be printable on the
// board must provide a summary() method." Then we can write ONE function
// that accepts any of them, and even mix them in a single Vec.

// --- The trait ---------------------------------------------------------
// Any type implementing DisplaySummary promises to provide `summary()`.
trait DisplaySummary {
    fn summary(&self) -> String;
}

// --- Flight ------------------------------------------------------------
struct Flight {
    number: String,
    destination: String,
    gate: u32,
    seats_available: u32,
}

impl Flight {
    // A constructor (inherent method, not part of any trait).
    fn new(number: &str, destination: &str, gate: u32, seats: u32) -> Self {
        Self {
            number: number.to_string(),
            destination: destination.to_string(),
            gate,
            seats_available: seats,
        }
    }

    // Another inherent method — only callable on a Flight, not on any
    // DisplaySummary in general.
    fn is_full(&self) -> bool {
        self.seats_available == 0
    }

    // Mutating method: decrements seats.
    fn check_in(&mut self) -> bool {
        if self.seats_available == 0 {
            false
        } else {
            self.seats_available -= 1;
            true
        }
    }
}

// Flight's summary: focus on route and seat count.
impl DisplaySummary for Flight {
    fn summary(&self) -> String {
        format!(
            "[FLIGHT]    {} -> {} | gate {} | {} seats left",
            self.number, self.destination, self.gate, self.seats_available
        )
    }
}

// --- Passenger ---------------------------------------------------------
struct Passenger {
    name: String,
    seat: String,
    frequent_flyer: bool,
}

impl Passenger {
    fn new(name: &str, seat: &str, ff: bool) -> Self {
        Self { name: name.to_string(), seat: seat.to_string(), frequent_flyer: ff }
    }
}

// Passenger's summary is written COMPLETELY DIFFERENTLY from Flight's:
// different wording, different fields, even a conditional tag. That's
// the whole point — each type decides how to present itself.
impl DisplaySummary for Passenger {
    fn summary(&self) -> String {
        let tag = if self.frequent_flyer { " *FF*" } else { "" };
        format!("[PASSENGER] {} in seat {}{}", self.name, self.seat, tag)
    }
}

// --- Baggage -----------------------------------------------------------
struct Baggage {
    tag: String,
    weight_kg: f64,
}

impl Baggage {
    fn new(tag: &str, weight_kg: f64) -> Self {
        Self { tag: tag.to_string(), weight_kg }
    }
}

// Baggage formats its summary yet differently — includes a heavy-bag
// warning that neither Flight nor Passenger would ever produce.
impl DisplaySummary for Baggage {
    fn summary(&self) -> String {
        let note = if self.weight_kg > 23.0 { "  !!HEAVY!!" } else { "" };
        format!("[BAGGAGE]   tag {} | {:.1} kg{}", self.tag, self.weight_kg, note)
    }
}

// --- One function that accepts ANY DisplaySummary ----------------------
//
// `&dyn DisplaySummary` means "a reference to some value whose concrete
// type we don't know at compile time, but which implements this trait."
// The compiler looks up the right `summary()` at runtime via a vtable.
// This is how we get heterogeneous collections in Rust.
fn print_to_board(item: &dyn DisplaySummary) {
    println!("BOARD: {}", item.summary());
}

// --- main --------------------------------------------------------------
fn main() {
    let mut flight = Flight::new("AA1234", "San Francisco", 17, 2);
    let alice      = Passenger::new("Alice Nguyen", "14A", true);
    let bob        = Passenger::new("Bob Patel",    "22C", false);
    let bag1       = Baggage::new("BAG-0001", 18.5);
    let bag2       = Baggage::new("BAG-0002", 27.2); // triggers HEAVY note

    println!("=== Single function, three different types ===");
    // Calling the SAME function with THREE different types. This would
    // be impossible without a trait.
    print_to_board(&flight);
    print_to_board(&alice);
    print_to_board(&bag1);

    println!();
    println!("=== Mixed Vec, one loop ===");
    // Store a heterogeneous mix in a single Vec<&dyn DisplaySummary>,
    // then iterate once. This is the headline benefit of traits.
    let board: Vec<&dyn DisplaySummary> = vec![&flight, &alice, &bob, &bag1, &bag2];
    for item in &board {
        // Every `item` gets dispatched to its own type's `summary()`.
        println!("ITEM: {}", item.summary());
    }

    println!();
    println!("=== Inherent methods only on Flight ===");
    // These methods live on Flight alone — they are NOT part of the
    // DisplaySummary trait, so they can't be called through `&dyn`.
    println!("Flight full? {}", flight.is_full());
    println!("Checking in a passenger...");
    flight.check_in();
    println!("Checking in another passenger...");
    flight.check_in();
    println!("Flight full now? {}", flight.is_full());
}
