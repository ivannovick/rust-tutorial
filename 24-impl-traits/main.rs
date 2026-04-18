// Lesson 24 — impl, methods, and traits
//
// Methods are attached to a type via an `impl` block. Traits describe a
// set of methods that any type can implement — similar to interfaces in
// other languages.

// A trait: any type implementing it can produce a summary string.
trait DisplaySummary {
    fn summary(&self) -> String;
}

// --- Flight -------------------------------------------------------------
struct Flight {
    number: String,
    destination: String,
    gate: u32,
    seats_available: u32,
}

// Methods on Flight.
impl Flight {
    // Read-only method: `&self`.
    fn is_full(&self) -> bool {
        self.seats_available == 0
    }

    // Mutating method: `&mut self`. Decrements seats_available (but not
    // below zero). Returns true if check-in succeeded.
    fn check_in(&mut self) -> bool {
        if self.seats_available == 0 {
            false
        } else {
            self.seats_available -= 1;
            true
        }
    }
}

// Implement DisplaySummary for Flight.
impl DisplaySummary for Flight {
    fn summary(&self) -> String {
        format!(
            "Flight {} to {} - Gate {} - {} seats left",
            self.number, self.destination, self.gate, self.seats_available
        )
    }
}

// --- Passenger ----------------------------------------------------------
struct Passenger {
    name: String,
    status: String, // kept simple; see Lesson 20 for a real enum
}

// Implement DisplaySummary for Passenger too.
impl DisplaySummary for Passenger {
    fn summary(&self) -> String {
        format!("Passenger {} ({})", self.name, self.status)
    }
}

// --- main ---------------------------------------------------------------
fn main() {
    let mut flight = Flight {
        number: String::from("AA1234"),
        destination: String::from("San Francisco"),
        gate: 17,
        seats_available: 2,
    };

    let alice = Passenger {
        name: String::from("Alice Nguyen"),
        status: String::from("Confirmed"),
    };

    // Because both Flight and Passenger implement DisplaySummary, we can
    // treat them uniformly via `&dyn DisplaySummary` (trait object).
    let items: Vec<&dyn DisplaySummary> = vec![&flight, &alice];
    for item in items {
        println!("[summary] {}", item.summary());
    }

    // Use Flight's own methods.
    println!("Flight {} full? {}", flight.number, flight.is_full());

    println!("Checking in a passenger on {}...", flight.number);
    flight.check_in();
    println!("Flight {} full? {}", flight.number, flight.is_full());

    println!("Checking in a passenger on {}...", flight.number);
    flight.check_in();
    println!("Flight {} full? {}", flight.number, flight.is_full());
}
