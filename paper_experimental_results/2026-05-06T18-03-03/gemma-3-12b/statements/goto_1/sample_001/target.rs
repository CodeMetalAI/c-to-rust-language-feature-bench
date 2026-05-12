fn main() {
    let mut acc = 0;
    acc = 15;
    for _i in 0..5 {
        acc += 1;
        if acc < 15 {
            acc = -15;
        }
        acc = 15;
    }

    if acc == 20 {
        0
    } else {
        1
    }
}