fn main() {
    let s = "12345";
    let mut acc = 0;
    let mut chars = s.chars();

    while let Some(c) = chars.next() {
        if c == '\0' {
            break;
        }
        acc += 1;
    }

    std::process::exit(if acc == 5 { 0 } else { 1 });
}