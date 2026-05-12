fn main() {
    let x = 10;
    let y = 20;

    // Simulate pointers as indices within their own single-element objects.
    let x_index = 0usize;
    let y_index = 0usize;

    let px = x_index;
    let py = y_index;

    let px_end = px + 1;
    let py_end = py + 1;

    if px != x_index {
        std::process::exit(1);
    }

    if !(px_end > px) {
        std::process::exit(2);
    }

    if px_end != (x_index + 1) {
        std::process::exit(3);
    }

    if !(py_end > py) {
        std::process::exit(4);
    }

    // Use x and y to avoid unused variable warnings (though Rust doesn't warn in release).
    let _ = (x, y);
}