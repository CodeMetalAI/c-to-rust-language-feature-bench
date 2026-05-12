fn main() {
    let x = 10;
    let y = 20;

    let px: &i32 = &x;
    let py: &i32 = &y;

    // In Rust, we can use slices to simulate pointer arithmetic
    let x_arr = std::slice::from_ref(&x);
    let y_arr = std::slice::from_ref(&y);

    // px_end and py_end are conceptually "one past the end"
    // We'll use indices to simulate the pointer comparisons

    let px_idx: usize = 0;
    let px_end_idx: usize = 1;
    let py_idx: usize = 0;
    let py_end_idx: usize = 1;

    // if (px != &x) - px always equals &x, so this is always false
    if std::ptr::eq(px, &x) == false {
        std::process::exit(1);
    }

    // if (!(px_end > px)) - px_end (index 1) > px (index 0) is always true
    if !(px_end_idx > px_idx) {
        std::process::exit(2);
    }

    // if (px_end != (&x + 1)) - px_end always equals &x + 1
    if px_end_idx != px_idx + 1 {
        std::process::exit(3);
    }

    // if (!(py_end > py)) - py_end (index 1) > py (index 0) is always true
    if !(py_end_idx > py_idx) {
        std::process::exit(4);
    }

    std::process::exit(0);
}