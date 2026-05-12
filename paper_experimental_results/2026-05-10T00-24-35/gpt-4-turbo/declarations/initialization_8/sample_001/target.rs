fn main() {
    // Equivalent to char s[] = "abc";
    let mut s = ['a', 'b', 'c', '\0'];

    // Equivalent to char t[3] = "abc";
    let mut t = ['a', 'b', 'c'];

    if s[0] != 'a' || s[1] != 'b' || s[2] != 'c' || s[3] != '\0' {
        std::process::exit(1);
    }

    if t[0] != 'a' || t[1] != 'b' || t[2] != 'c' {
        std::process::exit(2);
    }

    // Modify array elements
    s[0] = 'x';
    t[2] = 'y';

    if s[0] != 'x' {
        std::process::exit(3);
    }
    if t[2] != 'y' {
        std::process::exit(4);
    }

    // Equivalent to char *p = "abc";
    let p = "abc";

    if p.chars().nth(0) != Some('a') || p.chars().nth(1) != Some('b') || p.chars().nth(2) != Some('c') || p.chars().nth(3) != None {
        std::process::exit(5);
    }
}