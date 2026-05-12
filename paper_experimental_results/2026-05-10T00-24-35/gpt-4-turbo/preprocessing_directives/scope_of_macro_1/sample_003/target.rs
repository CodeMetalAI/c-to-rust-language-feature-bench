fn main() {
    const TABSIZE: usize = 100; // Equivalent to #define TABSIZE 100 in C/C++
    let mut a = [0; TABSIZE]; // Equivalent to int a[TABSIZE] in C/C++

    // Since TABSIZE is a constant in Rust, it cannot be changed after it's defined.
    // Thus, we simulate the C/C++ behavior by defining a mutable variable.
    let mut tabsize = 7; // Equivalent to int TABSIZE and TABSIZE = 7 in C/C++

    // Check the size of the array `a`
    if a.len() != 100 {
        std::process::exit(1);
    }

    // Check the value of the mutable variable `tabsize`
    if tabsize != 7 {
        std::process::exit(2);
    }

    // If all checks pass, exit normally
    std::process::exit(0);
}