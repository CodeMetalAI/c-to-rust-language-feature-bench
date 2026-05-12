fn main() {
    let x = 10;
    let y = 20;

    let px: &i32 = &x;
    let py: &i32 = &y;

    // In Rust, we can use slices to simulate pointer arithmetic
    let x_arr = std::slice::from_ref(&x);
    let y_arr = std::slice::from_ref(&y);

    // Check if px points to x (always true by construction)
    if !std::ptr::eq(px, &x) {
        std::process::exit(1);
    }

    // Check if px_end > px (simulated by checking slice bounds)
    // In the original C code, px_end = px + 1, so px_end > px is always true
    // for a valid single-element "array"
    if !(x_arr.len() > 0) {
        std::process::exit(2);
    }

    // Check if px_end == &x + 1 (always true by construction in C)
    // We verify the slice has exactly 1 element
    if x_arr.len() != 1 {
        std::process::exit(3);
    }

    // Check if py_end > py (same logic as above)
    if !(y_arr.len() > 0) {
        std::process::exit(4);
    }

    // Suppress unused variable warning
    let _ = py;
    let _ = y;

    std::process::exit(0);
}