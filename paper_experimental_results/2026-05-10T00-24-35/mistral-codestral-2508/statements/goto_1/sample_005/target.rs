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

    let exit_code = if acc == 20 { 0 } else { 1 };
    std::process::exit(exit_code);
}