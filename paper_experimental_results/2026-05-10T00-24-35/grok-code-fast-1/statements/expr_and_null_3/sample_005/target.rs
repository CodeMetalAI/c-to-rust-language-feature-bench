fn main() {
    let mut x = 0;
    // The goto skips setting x to 1, so x remains 0
    x += 1;
    std::process::exit(if x == 1 { 0 } else { 1 });
}