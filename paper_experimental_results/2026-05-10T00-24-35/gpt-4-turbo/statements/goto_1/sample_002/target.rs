fn main() {
    let mut acc = 0;
    let mut i = 0;

    while i < 5 {
        if i == 0 {
            acc = 15;
        } else {
            acc += 1;
            if acc < 15 {
                acc = -15;
            }
        }
        i += 1;
    }

    std::process::exit(if acc == 20 { 0 } else { 1 });
}