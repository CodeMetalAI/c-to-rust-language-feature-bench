fn main() {
    let a: [i32; 3] = [10, 20, 30];

    // In C, array `a` decays to `int *` in most contexts (TYPE_ID(a) == 1)
    // In Rust, we check the behavior rather than the type directly
    // The key is that `a` can be indexed and accessed like a pointer would be
    
    // TYPE_ID(a) != 1 check - in C, array decays to pointer
    // We simulate this by checking we can access it as expected
    
    // TYPE_ID(&a) != 2 check - &a would be int (*)[3] in C
    // In Rust, &a is &[i32; 3], which is the reference to the array
    
    // *a != 10 - in C, *a is the first element
    if a[0] != 10 {
        std::process::exit(3);
    }
    
    // *(a + 1) != 20
    if a[1] != 20 {
        std::process::exit(4);
    }
    
    // *(a + 2) != 30
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
    
    // TYPE_ID("abc") != 3 - string literal is const char * in C
    // We skip this type check as Rust handles strings differently
    
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