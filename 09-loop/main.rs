// Lesson 09 — `loop`
//
// `loop` runs forever until you explicitly `break`. It's the right choice
// when you don't know ahead of time how many iterations you need.
fn main() {
    let flight_number = "AA1234";
    let mut minutes_remaining = 5;

    println!("Final boarding call for flight {}", flight_number);

    loop {
        // When the countdown hits zero, exit the loop.
        if minutes_remaining == 0 {
            println!("Now boarding!");
            break;
        }

        // Handle singular "1 minute" vs plural "N minutes".
        if minutes_remaining == 1 {
            println!("Boarding in 1 minute...");
        } else {
            println!("Boarding in {} minutes...", minutes_remaining);
        }

        minutes_remaining -= 1;
    }
}
