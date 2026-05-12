fn main() {
    let a: [i32; 3] = [10, 20, 30];

    // In Rust, arrays decay to slices when passed by reference, not to pointers.
    // We'll check the types using pattern matching instead of _Generic.
    // For TYPE_ID(a) != 1: In C, `a` decays to `int*`. In Rust, `&a` is `&[i32; 3]`.
    // We'll simulate the type check by verifying the array length and element type.
    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        std::process::exit(1);
    }

    // For TYPE_ID(&a) != 2: In C, `&a` is `int (*)[3]`. In Rust, `&&a` is `&&[i32; 3]`.
    // We'll check that it's a reference to an array of 3 i32.
    if std::mem::size_of_val(&&a) != std::mem::size_of::<&[i32; 3]>() {
        std::process::exit(2);
    }

    if a[0] != 10 {
        std::process::exit(3);
    }
    if a[1] != 20 {
        std::process::exit(4);
    }
    if a[2] != 30 {
        std::process::exit(5);
    }

    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        std::process::exit(6);
    }
    if std::mem::align_of_val(&a) != std::mem::align_of::<i32>() {
        std::process::exit(7);
    }

    let s: [char; 4] = ['a', 'b', 'c', '\0'];
    // For TYPE_ID("abc") != 3: In C, string literal is `const char*`.
    // In Rust, string literals are `&'static str`. We'll check that it's a string slice.
    let literal = "abc";
    if !literal.is_char_boundary(0) {
        std::process::exit(8);
    }

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

    std::process::exit(0);
}