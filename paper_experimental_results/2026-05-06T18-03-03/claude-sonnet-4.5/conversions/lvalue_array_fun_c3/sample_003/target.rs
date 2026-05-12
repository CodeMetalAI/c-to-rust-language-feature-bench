fn main() {
    let a: [i32; 3] = [10, 20, 30];

    // TYPE_ID(a) checks - in C, array decays to pointer in most contexts
    // In Rust, we explicitly check the types
    // TYPE_ID(a) != 1 means a should decay to *int
    // This is implicitly true in the checks below

    // TYPE_ID(&a) != 2 means &a should be pointer to array
    // In Rust, &a is &[i32; 3], which is the equivalent
    let _ref_to_array: &[i32; 3] = &a;

    // *a != 10 - first element
    if a[0] != 10 {
        std::process::exit(3);
    }

    // *(a + 1) != 20 - second element
    if a[1] != 20 {
        std::process::exit(4);
    }

    // *(a + 2) != 30 - third element
    if a[2] != 30 {
        std::process::exit(5);
    }

    // sizeof a != 3 * sizeof(int)
    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        std::process::exit(6);
    }

    // _Alignof(a) != _Alignof(int)
    if std::mem::align_of_val(&a) != std::mem::align_of::<i32>() {
        std::process::exit(7);
    }

    let s: [u8; 4] = [b'a', b'b', b'c', b'\0'];

    // TYPE_ID("abc") != 3 - string literal is const char*
    // This is a compile-time type check in C, always passes

    if s[0] != b'a' {
        std::process::exit(9);
    }
    if s[1] != b'b' {
        std::process::exit(10);
    }
    if s[2] != b'c' {
        std::process::exit(11);
    }
    if s[3] != b'\0' {
        std::process::exit(12);
    }

    std::process::exit(0);
}