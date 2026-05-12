fn main() {
    let x: i32 = 42;
    let p: &i32 = &x;

    // In safe Rust, a reference to x is always equal to itself
    // This check is equivalent to checking if p points to x
    if !std::ptr::eq(p, &x) {
        std::process::exit(1);
    }

    // In safe Rust, we can't do pointer arithmetic directly on references
    // The original code checks if (p + 1) == (&x + 1), which would always be true
    // since p == &x. We simulate this by using a slice.
    let arr = [x];
    let p_slice = &arr[..];
    let p_end1 = &p_slice[1..]; // slice starting at index 1 (past the end)
    let p_end2 = &arr[1..];     // same thing

    // Check if the two "end" pointers are equal
    if !std::ptr::eq(p_end1.as_ptr(), p_end2.as_ptr()) {
        std::process::exit(2);
    }

    // Check if end pointer equals start pointer (should be false)
    if std::ptr::eq(p_end1.as_ptr(), p_slice.as_ptr()) {
        std::process::exit(3);
    }

    std::process::exit(0);
}