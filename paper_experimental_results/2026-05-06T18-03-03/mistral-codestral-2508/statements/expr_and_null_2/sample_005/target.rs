fn main() {
    let s = "12345";
    let acc = s.chars().count();
    if acc == 5 {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}