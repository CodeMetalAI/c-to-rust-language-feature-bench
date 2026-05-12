fn main() {
    let a = "abc";
    let b = "abc";

    if a.chars().nth(0) != Some('a') || a.chars().nth(1) != Some('b') || a.chars().nth(2) != Some('c') || a.len() != 3 {
        return std::process::exit(1);
    }
    if b.chars().nth(0) != Some('a') || b.chars().nth(1) != Some('b') || b.chars().nth(2) != Some('c') || b.len() != 3 {
        return std::process::exit(2);
    }

    if (a == b) != true && (a == b) != false {
        return std::process::exit(3);
    }

    std::process::exit(0);
}