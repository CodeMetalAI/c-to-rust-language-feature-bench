fn main() {
    let a: &'static str = "abc";
    let b = "abc";

    if a.chars().next() != Some('a') || a.chars().nth(1) != Some('b') || a.chars().nth(2) != Some('c') {
        return 1;
    }
    if b.chars().next() != Some('a') || b.chars().nth(1) != Some('b') || b.chars().nth(2) != Some('c') {
        return 2;
    }

    if a == b {
        return 3;
    }

    return 0;
}