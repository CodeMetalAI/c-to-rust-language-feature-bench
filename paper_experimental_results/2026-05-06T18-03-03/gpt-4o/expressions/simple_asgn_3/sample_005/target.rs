fn main() {
    let c: char = 'A';
    let p: &mut u8 = &mut 0u8;

    // Simulate the behavior of the original C code
    let cpp: &mut &u8 = &mut p;
    *cpp = &(c as u8);

    // Check if the value of `c` has changed
    if c != 'A' {
        std::process::exit(1);
    }

    std::process::exit(0);
}