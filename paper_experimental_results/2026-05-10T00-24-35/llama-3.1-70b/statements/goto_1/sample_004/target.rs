fn main() -> i32 {
    let mut acc = 0;
    let mut i = 0;
    loop {
        if i == 0 {
            acc = 15;
        }
        acc += 1;
        if acc < 15 {
            acc = -15;
        }
        i += 1;
        if i < 5 {
            continue;
        }
        break;
    }
    if acc == 20 {
        0
    } else {
        1
    }
}