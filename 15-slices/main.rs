// Lesson 15 — Slices
//
// A slice `&[T]` is a reference to a contiguous range of an array or vec.
// It borrows rather than owns. Written as `&arr[start..end]`.
fn main() {
    // Ten seat numbers on our flight.
    let seats: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Seats 1-3 are business class. `0..3` is exclusive of 3, so it
    // yields indexes 0, 1, 2 => the first three elements.
    let business_seats: &[i32] = &seats[0..3];

    // Seats 4-10 are economy. `3..` means "from index 3 to the end".
    let economy_seats: &[i32] = &seats[3..];

    // `{:?}` uses Debug formatting — works for slices, arrays, tuples, etc.
    println!("All seats:       {:?}", seats);
    println!("Business seats:  {:?}", business_seats);
    println!("Economy seats:   {:?}", economy_seats);
}
