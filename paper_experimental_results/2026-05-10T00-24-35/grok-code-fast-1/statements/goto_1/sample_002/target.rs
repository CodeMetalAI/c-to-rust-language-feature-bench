fn main() {
    let mut acc: i32 = 0;
    let mut skip = true;
    for _i in 0..5 {
        if !skip {
            acc += 1;
            if acc < 15 {
                acc = -15;
            }
        }
        skip = false;
        acc = 15;
    }
    std::process::exit(if acc == 20 { 0 } else { 1 });
}