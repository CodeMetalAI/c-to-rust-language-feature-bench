fn main() {
    let a = "abc";
    let b = "abc";

    if a.chars().nth(0).unwrap() != 'a' || a.chars().nth(1).unwrap() != 'b' || a.chars().nth(2).unwrap() != 'c' || a.chars().nth(3).unwrap_or('\0') != '\0' {
        return 1;
    }
    if b.chars().nth(0).unwrap() != 'a' || b.chars().nth(1).unwrap() != 'b' || b.chars().nth(2).unwrap() != 'c' || b.chars().nth(3).unwrap_or('\0') != '\0' {
        return 2;
    }

    if a == b {
        return 3;
    }

    return 0;
}