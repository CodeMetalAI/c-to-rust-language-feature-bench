fn main() {
    let a = "abc";
    let b = "abc";

    if a.chars().nth(0).unwrap() != 'a' || a.chars().nth(1).unwrap() != 'b' || a.chars().nth(2).unwrap() != 'c' || a.chars().nth(3).unwrap() != '\0' {
        return std::process::exit(1);
    }
    if b.chars().nth(0).unwrap() != 'a' || b.chars().nth(1).unwrap() != 'b' || b.chars().nth(2).unwrap() != 'c' || b.chars().nth(3).unwrap() != '\0' {
        return std::process::exit(2);
    }

    if (a == b) != false && (a == b) != true {
        return std::process::exit(3);
    }

    std::process::exit(0);
}