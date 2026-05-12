fn main() {
    let mut acc = 0;
    // Simulate goto first_time: jump to acc = 15; then continue (which does nothing since no loop)
    acc = 15;
    let exit_code = if acc == 20 { 0 } else { 1 };
    std::process::exit(exit_code);
}