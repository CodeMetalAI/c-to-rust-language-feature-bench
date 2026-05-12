fn main() {
    let mut acc: i32 = 0;
    let mut i: i32 = 0;
    let mut first = true;

    loop {
        if !first {
            if i >= 5 {
                break;
            }
            acc += 1;
            if acc < 15 {
                acc = -15;
            }
        }

        // first_time:
        acc = 15;

        // continue;
        i += 1;
        if i >= 5 {
            break;
        }
        first = false;
    }

    let exit_code = if acc == 20 { 0 } else { 1 };
    std::process::exit(exit_code);
}