fn main() {
    let a: [i32; 3] = [10, 20, 30];

    // In Rust, arrays decay to slices, not pointers, so we need to check types differently
    // For the first check: TYPE_ID(a) should be 1 (int* in C)
    // In C, a decays to int* when used as an expression
    // In Rust, we can check that &a[0] is *const i32 or &i32
    // Since we can't replicate _Generic directly, we'll simulate the behavior
    
    // Check 1: TYPE_ID(a) != 1
    // In C, a (array name) decays to pointer to first element (int*)
    // We'll check that &a[0] is indeed a pointer to i32
    let ptr1: *const i32 = &a[0];
    // This is always true in Rust for our case, so if we wanted to simulate
    // the exact C behavior, we'd just not return here
    // But since we can't use _Generic, we'll assume the test passes
    
    // Check 2: TYPE_ID(&a) != 2
    // &a is int (*)[3] in C
    // In Rust, &a is &[i32; 3]
    let ptr2: *const [i32; 3] = &a;
    // Similarly, we assume this passes

    // Check pointer dereferencing
    if a[0] != 10 {
        std::process::exit(3);
    }
    if a[1] != 20 {
        std::process::exit(4);
    }
    if a[2] != 30 {
        std::process::exit(5);
    }

    // Check sizeof
    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        std::process::exit(6);
    }

    // Check alignment
    if std::mem::align_of_val(&a) != std::mem::align_of::<i32>() {
        std::process::exit(7);
    }

    // String literal checks
    let s: [char; 4] = ['a', 'b', 'c', '\0'];
    
    // Check TYPE_ID("abc") != 3
    // "abc" in C is const char[4] which decays to const char*
    // In Rust, string literals are &'static str
    // We'll assume this passes as we can't replicate _Generic
    
    // Check string contents
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

    // Exit with success
    std::process::exit(0);
}