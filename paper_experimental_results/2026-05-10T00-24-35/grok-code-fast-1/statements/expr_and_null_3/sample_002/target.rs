fn main() {
    let mut x = 0;

    // The goto skips setting x to 1
    // So x remains 0

    x += 1;
    let exit_code = if x == 1 { 0 } else { 1 };
    std::process::exit(exit_code);
}