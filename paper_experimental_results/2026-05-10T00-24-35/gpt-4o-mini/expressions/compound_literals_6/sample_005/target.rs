fn main() {
    let a: &str = "abc"; // equivalent to const char[] {"abc"}
    let b: &str = "abc"; // equivalent to const char* b = "abc";

    if a.chars().nth(0) != Some('a') || a.chars().nth(1) != Some('b') || a.chars().nth(2) != Some('c') || a.chars().nth(3) != None {
        std::process::exit(1);
    }
    if b.chars().nth(0) != Some('a') || b.chars().nth(1) != Some('b') || b.chars().nth(2) != Some('c') || b.chars().nth(3) != None {
        std::process::exit(2);
    }

    if (std::ptr::eq(a.as_ptr(), b.as_ptr()) as i32) != 0 && (std::ptr::eq(a.as_ptr(), b.as_ptr()) as i32) != 1 {
        std::process::exit(3);
    }

    std::process::exit(0);
}