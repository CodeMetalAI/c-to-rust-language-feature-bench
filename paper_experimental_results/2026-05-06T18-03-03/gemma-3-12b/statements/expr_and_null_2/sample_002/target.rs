fn main() {
    let s = "12345";
    let mut acc = 0;
    for c in s.chars() {
        if c == '\0' {
            break;
        }
        acc += 1;
    }
    if acc == 5 {
        0
    } else {
        1
    }
}