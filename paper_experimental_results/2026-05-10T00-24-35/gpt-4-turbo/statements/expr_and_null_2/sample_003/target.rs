fn main() {
    let s = "12345";
    let mut acc = 0;
    for _ in s.chars() {
        acc += 1;
    }
    std::process::exit(if acc == 5 { 0 } else { 1 });
}