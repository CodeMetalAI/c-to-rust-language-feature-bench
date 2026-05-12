fn main() {
    let mut acc = 0;
    let mut i = 0;
    let mut first_entry = true;

    loop {
        if !first_entry {
            acc += 1;
            if acc < 15 {
                acc = -15;
            }
        }

        // first_time:
        acc = 15;
        first_entry = false;

        // continue equivalent - go to increment and check
        i += 1;
        if i > 5 {
            break;
        }
    }

    std::process::exit(if acc == 20 { 0 } else { 1 });
}