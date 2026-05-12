fn main() {
    let mut acc = 0;

    // Equivalent to the goto first_time; line.
    // The loop will handle the initial assignment of acc to 15.

    for i in 0..5 {
        acc += 1;
        if acc < 15 {
            acc = -15;
        }

        acc = 15; // Equivalent to the first_time: label
    }

    if acc == 20 {
        0
    } else {
        1
    }
}