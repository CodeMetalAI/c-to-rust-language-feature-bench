fn main() {
    let a: &str = "abc";
    let b: &str = "abc";

    if a.chars().nth(0) != Some('a') || a.chars().nth(1) != Some('b') ||
       a.chars().nth(2) != Some('c') || a.chars().nth(3) != None {
        std::process::exit(1);
    }
    if b.chars().nth(0) != Some('a') || b.chars().nth(1) != Some('b') ||
       b.chars().nth(2) != Some('c') || b.chars().nth(3) != None {
        std::process::exit(2);
    }

    let ptr_a = a.as_ptr();
    let ptr_b = b.as_ptr();

    let same = ptr_a == ptr_b;
    if !same {
        // nothing to do, just ensuring the comparison logic mirrors C++ `if ((a == b) != 0 && (a == b) != 1)`
    } else {
        std::process::exit(3);
    }
}