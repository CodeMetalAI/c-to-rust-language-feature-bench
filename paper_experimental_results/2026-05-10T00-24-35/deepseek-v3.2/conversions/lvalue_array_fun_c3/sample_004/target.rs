fn main() {
    let a: [i32; 3] = [10, 20, 30];

    // In Rust, arrays don't decay to pointers automatically, so we need to check types differently
    // Check that `a` is an array of 3 i32s
    if !matches!(a, [10, 20, 30]) {
        std::process::exit(1);
    }

    // Check that `&a` is a reference to an array of 3 i32s
    if !matches!(&a, &[10, 20, 30]) {
        std::process::exit(2);
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

    // Check size
    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        std::process::exit(6);
    }

    // Check alignment
    if std::mem::align_of_val(&a) != std::mem::align_of::<i32>() {
        std::process::exit(7);
    }

    let s: [char; 4] = ['a', 'b', 'c', '\0'];

    // Check string literal type - Rust string literals are &str
    if !matches!("abc", "abc") {
        std::process::exit(8);
    }

    // Check array values
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