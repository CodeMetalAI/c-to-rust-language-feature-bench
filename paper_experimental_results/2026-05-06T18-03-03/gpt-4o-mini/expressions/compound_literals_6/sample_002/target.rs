fn main() {
    let a = &["a", "b", "c", "\0"]; // Using an array of string slices
    let b = "abc";

    if a[0] != "a" || a[1] != "b" || a[2] != "c" || a[3] != "\0" {
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