fn main() {
    let mut acc = 0;
    'outer: loop {
        acc = 15;
        for _ in 0..5 {
            acc += 1;
            if acc < 15 {
                acc = -15;
                break 'outer;
            }
        }
    }
    std::process::exit(if acc == 20 { 0 } else { 1 });
}