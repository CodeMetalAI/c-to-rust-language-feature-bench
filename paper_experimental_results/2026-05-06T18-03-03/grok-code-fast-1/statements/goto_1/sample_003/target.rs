fn main() {
    let mut acc = 0;
    // Simulate goto first_time
    acc = 15;
    // The for loop
    for _i in 0..5 {
        acc += 1;
        if acc < 15 {
            acc = -15;
        }
        // first_time:
        acc = 15;
    }
    std::process::exit(if acc == 20 { 0 } else { 1 });
}