fn main() {
    let a: &str = "abc";
    let b: &str = "abc";

    if a.chars().nth(0) != Some('a') || a.chars().nth(1) != Some('b') || a.chars().nth(2) != Some('c') || a.chars().nth(3) != Some('\0') {
        std::process::exit(1);
    }
    if b.chars().nth(0) != Some('a') || b.chars().nth(1) != Some('b') || b.chars().nth(2) != Some('c') || b.chars().nth(3) != Some('\0') {
        std::process::exit(2);
    }

    let a_ptr: *const u8 = a.as_ptr();
    let b_ptr: *const u8 = b.as_ptr();

    if (a_ptr == b_ptr) != false && (a_ptr == b_ptr) != true {
        std::process::exit(3);
    }
}