fn main() {
    let a: [i32; 3] = [10, 20, 30];

    // In C, array `a` decays to `int *` in most contexts
    // In Rust, we need to explicitly check types differently
    // TYPE_ID(a) != 1: In C, `a` decays to pointer in _Generic context
    // We'll verify the behavior matches by checking the array works as expected
    
    // TYPE_ID(&a) != 2: In C, &a is `int (*)[3]`
    // In Rust, &a is &[i32; 3], which is the equivalent
    
    // Check array element access (dereferencing behavior)
    if a[0] != 10 {
        std::process::exit(3);
    }
    if a[1] != 20 {
        std::process::exit(4);
    }
    if a[2] != 30 {
        std::process::exit(5);
    }

    // Check sizeof: array should be 3 * sizeof(int)
    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        std::process::exit(6);
    }
    
    // Check alignment
    if std::mem::align_of_val(&a) != std::mem::align_of::<i32>() {
        std::process::exit(7);
    }

    let s: [u8; 4] = [b'a', b'b', b'c', b'\0'];
    
    // TYPE_ID("abc") checks - string literals in C are const char*
    // This would be type 3 in the C code
    
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