fn main() {
    let mut acc = 0;
    let mut i = 0;
    loop {
        acc += 1;
        if acc < 15 {
            acc = -15;
        }
        i += 1;
        if i == 5 {
            break;
        }
    }
    acc = 15;
    acc += 5;
    assert_eq!(acc, 20);
}