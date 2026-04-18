// Lesson 22 — Pattern matching
//
// `match` is Rust's powerful pattern-dispatch construct. It's exhaustive —
// every possible value must be covered or the program won't compile.
enum FlightStatus {
    OnTime,
    Delayed(u32),   // variant carries data: delay in minutes
    Cancelled,
    Boarding,
}

fn main() {
    // A list of (flight number, status) pairs to process.
    let ops = [
        ("AA1234", FlightStatus::OnTime),
        ("UA5678", FlightStatus::Delayed(45)),
        ("DL9012", FlightStatus::Cancelled),
        ("WN3456", FlightStatus::Boarding),
    ];

    for (flight, status) in &ops {
        let label = status_label(status);
        let action = staff_action(status);
        println!("Flight {} is {:<14} -> {}", flight, label, action);
    }
}

// Build a short display label for each variant.
fn status_label(status: &FlightStatus) -> String {
    match status {
        FlightStatus::OnTime         => String::from("OnTime"),
        // Bind the inner value of Delayed to `mins` so we can use it.
        FlightStatus::Delayed(mins)  => format!("Delayed({})", mins),
        FlightStatus::Cancelled      => String::from("Cancelled"),
        FlightStatus::Boarding       => String::from("Boarding"),
    }
}

// What staff should do for each status.
fn staff_action(status: &FlightStatus) -> String {
    match status {
        FlightStatus::OnTime => {
            String::from("Open gate and begin boarding on schedule.")
        }
        FlightStatus::Delayed(mins) => {
            format!(
                "Announce delay of {} minutes and reassure passengers.",
                mins
            )
        }
        FlightStatus::Cancelled => {
            String::from("Notify passengers and open rebooking desk.")
        }
        FlightStatus::Boarding => {
            String::from("Final call — close doors in 5 minutes.")
        }
    }
}
