fn main() {
    let mut acc = 0;
    loop {
        for i in 0..5 {
            acc += 1;
            if acc < 15 {
                acc = -15;
            }
            acc = 15;
        }
        break;
    }

    std::process::exit(if acc == 20 { 0 } else { 1 });
}