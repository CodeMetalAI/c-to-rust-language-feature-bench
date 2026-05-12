fn main() {
    let a = [10, 20, 30];

    // In Rust, we don't have a direct equivalent of C's _Generic, so we use type inference and checks.
    if a.as_ptr() as *const i32 != a.as_ptr() {
        std::process::exit(1);
    }
    if &a as *const _ as *const i32 != a.as_ptr() {
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

    let s = ['a', 'b', 'c', '\0'];
    let s_literal = "abc";
    if s_literal.as_ptr() as *const u8 != s_literal.as_ptr() {
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
}