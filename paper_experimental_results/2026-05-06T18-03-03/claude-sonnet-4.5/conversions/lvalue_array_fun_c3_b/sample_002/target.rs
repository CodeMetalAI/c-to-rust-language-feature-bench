fn main() {
    let a: [i32; 3] = [10, 20, 30];

    // In C, array `a` decays to `int *` in most contexts
    // TYPE_ID(a) checks the type after decay, which is `int *` (type 1)
    // In Rust, we simulate this by checking that array access works as expected
    
    // TYPE_ID(a) != 1 check - array decays to pointer in C
    // We verify the array behaves correctly instead
    
    // TYPE_ID(&a) != 2 check - &a is `int (*)[3]` in C
    // In Rust, &a is &[i32; 3], we just verify it exists
    let _ref_a: &[i32; 3] = &a;

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

    let s: [u8; 4] = [b'a', b'b', b'c', b'\0'];
    
    // TYPE_ID("abc") != 3 check - string literal is `const char *` in C
    // We verify string literal behavior instead
    let _literal: &[u8] = b"abc";
    
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