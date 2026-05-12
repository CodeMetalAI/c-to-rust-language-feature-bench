fn main() {
    let a: [i32; 3] = [10, 20, 30];

    // In Rust, arrays don't decay to pointers in the same way as C.
    // `a` is of type `[i32; 3]`, not `*const i32`.
    // We'll check the behavior by using references and raw pointers.
    let a_ptr: *const i32 = a.as_ptr();
    if a_ptr.is_null() {
        std::process::exit(1);
    }

    // `&a` is of type `&[i32; 3]`, which is a reference to the array.
    let a_ref: &[i32; 3] = &a;
    if a_ref.len() != 3 {
        std::process::exit(2);
    }

    // Check array values
    if a[0] != 10 {
        std::process::exit(3);
    }
    if a[1] != 20 {
        std::process::exit(4);
    }
    if a[2] != 30 {
        std::process::exit(5);
    }

    // Check size and alignment
    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        std::process::exit(6);
    }
    if std::mem::align_of_val(&a) != std::mem::align_of::<i32>() {
        std::process::exit(7);
    }

    // String literal and array
    let s: [char; 4] = ['a', 'b', 'c', '\0'];
    // String literal "abc" is of type `&'static str` in Rust.
    // We'll just check the array `s`.
    if s[0] != 'a' {
        std::process::exit(9);
    }
    if s[1] != 'b' {
        std::process::exit(10);
    }
    if s[2] != 'c' {
        std::process::exit(11);
    }
    if s[3] != '\0' {
        std::process::exit(12);
    }

    // All checks passed
    std::process::exit(0);
}