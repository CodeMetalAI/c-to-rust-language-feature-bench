fn main() -> i32 {
    let mut acc = 0;
    loop {
        acc += 1;
        if acc < 15 {
            acc = -15;
        }
        if acc == 20 {
            break;
        }
    }
    if acc == 20 {
        0
    } else {
        1
    }
}