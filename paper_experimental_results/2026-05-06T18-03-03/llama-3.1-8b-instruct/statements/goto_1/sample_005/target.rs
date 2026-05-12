fn main() {
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
    acc = 15;
    loop {
        acc += 1;
        if acc == 20 {
            break;
        }
    }
    println!("{}", acc == 20);
}