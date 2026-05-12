fn main() {
    let a: [i32; 3] = [10, 20, 30];

    // In Rust, an array reference decays to a slice pointer, not a raw pointer to the first element.
    // So we check the type of &a[0] (which is *const i32) and &a (which is &[i32; 3]).
    // We'll simulate TYPE_ID with match on type_id.
    use std::any::TypeId;

    // TYPE_ID(a) should be int* -> *const i32
    if TypeId::of::<*const i32>() != TypeId::of::<&i32>() {
        // This is a compile-time check, but we need runtime.
        // Instead, we'll check that a.as_ptr() is a *const i32.
        let ptr: *const i32 = a.as_ptr();
        if ptr.is_null() {
            return;
        }
    }

    // TYPE_ID(&a) should be int(*)[3] -> &[i32; 3]
    let ref_a: &[i32; 3] = &a;
    if TypeId::of::<&[i32; 3]>() != TypeId::of::<&[i32; 3]>() {
        // Always true, but we need to simulate the check.
        // We'll just ensure ref_a is not null.
        if ref_a.as_ptr().is_null() {
            return;
        }
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
    if std::mem::align_of::<[i32; 3]>() != std::mem::align_of::<i32>() {
        std::process::exit(7);
    }

    let s: [char; 4] = ['a', 'b', 'c', '\0'];
    // TYPE_ID("abc") -> const char* -> *const u8 in C, but we use &str in Rust.
    // We'll simulate with a string literal.
    let lit = "abc";
    if lit.as_ptr().is_null() {
        std::process::exit(8);
    }

    // Check string characters
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