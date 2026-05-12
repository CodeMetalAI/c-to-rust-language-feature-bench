fn main() {
    let s = "12345";
    let acc = s.chars().count();
    std::process::exit(acc == 5 ? 0 : 1);
}