fn main() {
    let mut acc = 0;
    let mut i = 0;
    while i < 5 {
        if i > 0 {
            acc += 1;
            if acc < 15 {
                acc = -15;
            }
        }
        acc = 15;
        i += 1;
    }
    if acc == 20 {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}