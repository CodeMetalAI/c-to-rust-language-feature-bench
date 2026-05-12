fn main() {
    let mut acc = 0;
    let mut first_time = true;

    for _ in 0..5 {
        if first_time {
            first_time = false;
        } else {
            acc += 1;
            if acc < 15 {
                acc = -15;
            }
        }
        acc = 15;
    }

    std::process::exit(if acc == 20 { 0 } else { 1 });
}