use std::process::exit;

fn main() {
    let mut acc: i32 = 0;
    let mut i: i32 = 0;

    // goto first_time;
    acc = 15;
    // continue; -> increment and condition of the for loop
    i += 1;

    while i < 5 {
        acc += 1;
        if acc < 15 {
            acc = -15;
        }
        // first_time label
        acc = 15;
        i += 1;
    }

    if acc == 20 {
        exit(0);
    } else {
        exit(1);
    }
}