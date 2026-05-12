fn main() {
    let x = 10;
    let y = 20;

    let px = &x;
    let py = &y;

    // Rust does not allow pointer arithmetic like C++, so we use slice bounds to achieve similar checks.
    let px_range = std::slice::from_ref(px);
    let py_range = std::slice::from_ref(py);

    let px_end = &px_range[1] as *const _;
    let py_end = &py_range[1] as *const _;

    if px != &x {
        std::process::exit(1);
    }

    if px_end <= px {
        std::process::exit(2);
    }

    // Checking if end pointer calculated with slicing is equivalent to adding one to the base pointer in C/C++.
    if px_end != unsafe { px.offset(1) } {
        std::process::exit(3);
    }

    if py_end <= py {
        std::process::exit(4);
    }

    std::process::exit(0);
}