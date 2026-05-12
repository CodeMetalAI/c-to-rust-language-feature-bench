fn main() {
    let s = "12345";
    let acc = s.chars().count();
    std::process::exit(if acc == 5 { 0 } else { 1 });
}