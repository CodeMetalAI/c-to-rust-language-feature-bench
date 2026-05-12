use std::process::exit;

fn main() {
    let s = b"12345\0";
    let mut acc = 0;
    let mut idx = 0;
    loop {
        let c = s[idx];
        idx += 1;
        if c == 0 {
            break;
        }
        acc += 1;
    }
    if acc == 5 {
        exit(0);
    } else {
        exit(1);
    }
}