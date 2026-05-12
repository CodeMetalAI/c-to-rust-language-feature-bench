fn main() {
    let mut acc = 0;
    loop {
        acc = 15;
        for i in 0..5 {
            acc += 1;
            if acc < 15 {
                acc = -15;
            }
        }
        break;
    }

    if acc == 20 {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}