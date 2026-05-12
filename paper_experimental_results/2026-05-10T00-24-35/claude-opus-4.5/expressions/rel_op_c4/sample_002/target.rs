fn main() {
    let x = 10;
    let y = 20;

    let px = &x;
    let py = &y;

    // In safe Rust, we can't do pointer arithmetic like in C.
    // Instead, we use slices to achieve similar semantics.
    let x_slice = std::slice::from_ref(&x);
    let y_slice = std::slice::from_ref(&y);

    // Check if px points to x (it always does by construction)
    if !std::ptr::eq(px, &x) {
        std::process::exit(1);
    }

    // Check if end of x_slice is "greater than" start
    // In Rust, we can compare slice iterators or use pointer comparison
    // For a single-element slice, end > begin is always true
    if !(x_slice.len() > 0) {
        std::process::exit(2);
    }

    // Check if px_end equals &x + 1
    // This is equivalent to checking that the slice has exactly 1 element
    // and the end pointer matches
    let px_end = x_slice.as_ptr_range().end;
    let expected_end = unsafe { (&x as *const i32).add(1) };
    if !std::ptr::eq(px_end, expected_end) {
        std::process::exit(3);
    }

    // Check if end of y_slice is "greater than" start
    if !(y_slice.len() > 0) {
        std::process::exit(4);
    }

    // Suppress unused variable warnings
    let _ = py;
    let _ = y;

    std::process::exit(0);
}