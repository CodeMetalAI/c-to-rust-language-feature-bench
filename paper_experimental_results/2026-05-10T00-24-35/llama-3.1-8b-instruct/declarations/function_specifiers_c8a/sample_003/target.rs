fn main() {
    let v = if 0 { 123 } else { std::process::exit(0) };
    std::process::exit(1);
}