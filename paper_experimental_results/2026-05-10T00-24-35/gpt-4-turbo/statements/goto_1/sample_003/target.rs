fn main() {
    let mut acc = 0;

    for i in 0..5 {
        if i == 0 {
            acc = 15;
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