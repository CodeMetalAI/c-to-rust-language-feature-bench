fn main() {
    let mut acc = 0;
    loop {
        acc += 1;
        if acc >= 15 {
            acc = -15;
        } else {
            break;
        }
    }
    acc = 15;
    std::process::exit(match acc {
        20 => 0,
        _ => 1,
    });
}