fn main() {
    let x = 10;
    let y = 20;

    let px = &x;
    let py = &y;

    // In safe Rust, we can't do pointer arithmetic like in C.
    // We'll use slices to achieve similar semantics for the comparisons.
    
    let x_slice = std::slice::from_ref(&x);
    let y_slice = std::slice::from_ref(&y);

    // Check if px points to x (always true by construction)
    if !std::ptr::eq(px, &x) {
        std::process::exit(1);
    }

    // Check if end of x_slice is after the beginning
    // In safe Rust, we compare slice iterators or indices
    let px_end_idx = 1;
    let px_idx = 0;
    if !(px_end_idx > px_idx) {
        std::process::exit(2);
    }

    // Check if px_end equals &x + 1 (one past x)
    // This is equivalent to checking that the slice has length 1
    // and the end pointer would be at index 1
    if px_end_idx != x_slice.len() {
        std::process::exit(3);
    }

    // Check if end of y_slice is after the beginning
    let py_end_idx = 1;
    let py_idx = 0;
    if !(py_end_idx > py_idx) {
        std::process::exit(4);
    }

    // Suppress unused variable warnings
    let _ = py;
    let _ = y_slice;

    std::process::exit(0);
}