fn main() {
    let x: i32 = 42;
    let p: &i32 = &x;

    // In safe Rust, a reference to x is always equal to another reference to x
    // This check is equivalent to checking if p points to x
    if !std::ptr::eq(p, &x) {
        std::process::exit(1);
    }

    // In Rust, we can't do pointer arithmetic on references in safe code
    // But we can simulate the behavior using slices
    let arr = [x];
    let p_slice = &arr[..];
    let p_end1 = &arr[1..]; // slice starting after the first element
    let p_end2 = &arr[1..]; // same slice

    // Check if the two "end" pointers are equal (they point to the same location)
    if p_end1.as_ptr() != p_end2.as_ptr() {
        std::process::exit(2);
    }

    // Check if end pointer equals start pointer (it shouldn't)
    if p_end1.as_ptr() == p_slice.as_ptr() {
        std::process::exit(3);
    }

    std::process::exit(0);
}