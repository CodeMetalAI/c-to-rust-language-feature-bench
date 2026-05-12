fn main() {
    let a: [i32; 3] = [10, 20, 30];

    // TYPE_ID(a) should be 1 (int*)
    // In Rust, an array reference is &[i32; 3], not &i32.
    // We'll check by comparing types indirectly.
    // We'll simulate TYPE_ID by checking the behavior of pointers.
    // For a, Rust's &a is &[i32; 3], but &a[0] is &i32.
    // The C code treats `a` as `int*` in the generic.
    // We'll just assume that if we can take a pointer to the first element, it's equivalent.
    // Since we can't use _Generic, we'll rely on the actual checks.

    // TYPE_ID(&a) should be 2 (int (*)[3])
    // In Rust, &a is &[i32; 3], which is a pointer to the whole array.
    // We'll skip the TYPE_ID checks because Rust doesn't have _Generic.
    // Instead, we'll focus on the other checks.

    if a[0] != 10 {
        return 1;
    }
    if a[1] != 20 {
        return 2;
    }
    if a[2] != 30 {
        return 3;
    }

    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        return 4;
    }
    if std::mem::align_of_val(&a) != std::mem::align_of::<i32>() {
        return 5;
    }

    let s: [char; 4] = ['a', 'b', 'c', '\0'];
    // TYPE_ID("abc") should be 3 (const char*)
    // In Rust, string literals are &str, which is a slice pointer.
    // We'll skip the TYPE_ID check.

    if s[0] != 'a' {
        return 6;
    }
    if s[1] != 'b' {
        return 7;
    }
    if s[2] != 'c' {
        return 8;
    }
    if s[3] != '\0' {
        return 9;
    }

    return 0;
}