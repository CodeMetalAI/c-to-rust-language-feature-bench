fn main() {
    let mut acc = 0;
    for i in 0..5 {
        if i == 0 {
            acc = 15;
            continue;
        }
        acc += 1;
        if acc < 15 {
            acc = -15;
        }
        acc = 15;
    }
    let exit_code = if acc == 20 { 0 } else { 1 };
    std::process::exit(exit_code);
}