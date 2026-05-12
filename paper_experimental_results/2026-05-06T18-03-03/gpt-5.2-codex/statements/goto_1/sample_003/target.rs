fn main() {
    let mut acc: i32 = 0;
    let mut i: i32 = 0;
    let mut started = false;

    loop {
        if started {
            if i >= 5 {
                break;
            }
            acc += 1;
            if acc < 15 {
                acc = -15;
            }
            // first_time label
        } else {
            started = true;
            // goto first_time
        }

        acc = 15;
        // continue
        i += 1;
        if i >= 5 {
            break;
        }
    }

    let code = if acc == 20 { 0 } else { 1 };
    std::process::exit(code);
}