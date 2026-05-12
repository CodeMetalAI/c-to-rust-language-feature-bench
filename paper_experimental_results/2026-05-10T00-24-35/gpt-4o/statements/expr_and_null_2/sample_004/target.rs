fn main() {
    let mut s = "12345".chars();
    let mut acc = 0;
    while let Some(c) = s.next() {
        acc += 1;
    }
    std::process::exit(if acc == 5 { 0 } else { 1 });
}