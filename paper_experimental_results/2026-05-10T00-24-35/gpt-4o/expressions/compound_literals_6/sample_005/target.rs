fn main() {
    let a: &[u8] = b"abc";
    let b: &str = "abc";

    // Check if a matches "abc"
    if a[0] != b'a' || a[1] != b'b' || a[2] != b'c' || a.len() != 3 {
        std::process::exit(1);
    }

    // Check if b matches "abc"
    if b.chars().nth(0) != Some('a') || b.chars().nth(1) != Some('b') || b.chars().nth(2) != Some('c') || b.len() != 3 {
        std::process::exit(2);
    }

    // Since 'a' is a byte slice and 'b' is a string slice, comparing their addresses directly doesn't make sense in Rust.
    // Instead, we check the content's equality, as Rust does not expose address comparison like C/C++.
    if a != b.as_bytes() {
        std::process::exit(3);
    }

    std::process::exit(0);
}