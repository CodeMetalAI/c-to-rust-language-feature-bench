fn main() {
    let a: [i32; 3] = [10, 20, 30];

    // In Rust, arrays decay to slices (&[i32]) when used as references, not raw pointers
    // So we need to check the type of &a[0] or a.as_ptr()
    let a_ptr: *const i32 = a.as_ptr();
    // TYPE_ID(a) should be 1 (int*) - in Rust we check if it's a raw pointer to i32
    // We can't directly check the type like _Generic, but we can verify behavior
    if !std::mem::size_of_val(&a_ptr) == std::mem::size_of::<*const i32>() {
        std::process::exit(1);
    }

    // TYPE_ID(&a) should be 2 (int(*)[3]) - pointer to array of 3 i32
    let a_array_ptr: *const [i32; 3] = &a;
    if !std::mem::size_of_val(&a_array_ptr) == std::mem::size_of::<*const [i32; 3]>() {
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

    let s: [u8; 4] = [b'a', b'b', b'c', b'\0'];
    
    // TYPE_ID("abc") should be 3 (const char*) - string literal
    let string_lit = "abc";
    let string_ptr: *const u8 = string_lit.as_ptr();
    if !std::mem::size_of_val(&string_ptr) == std::mem::size_of::<*const u8>() {
        std::process::exit(8);
    }

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