fn main() {
    let mut s = "12345".chars();
    let mut acc = 0;
    while s.next() != None {
        acc += 1;
    }
    std::process::exit(if acc == 5 { 0 } else { 1 });
}