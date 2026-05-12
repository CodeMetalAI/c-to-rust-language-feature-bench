fn main() {
    let mut acc: i32 = 0;

    // Simulate: goto first_time;
    acc = 15;
    // continue; -> goes to increment and condition
    let mut i: i32 = 0;
    loop {
        i += 1;
        if i >= 5 {
            break;
        }

        // Loop body before label
        acc += 1;
        if acc < 15 {
            acc = -15;
        }

        // first_time:
        acc = 15;
        // continue; -> back to increment and condition
    }

    if acc == 20 {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}