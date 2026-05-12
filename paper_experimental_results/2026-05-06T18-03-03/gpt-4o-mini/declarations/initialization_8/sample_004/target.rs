fn main() -> i32 {
    let mut s = ['a', 'b', 'c', '\0'];
    let t = ['a', 'b', 'c'];

    if s[0] != 'a' || s[1] != 'b' || s[2] != 'c' || s[3] != '\0' {
        return 1;
    }

    if t[0] != 'a' || t[1] != 'b' || t[2] != 'c' {
        return 2;
    }

    s[0] = 'x';
    let mut t = t; // Make t mutable to change its contents
    t[2] = 'y';

    if s[0] != 'x' {
        return 3;
    }
    if t[2] != 'y' {
        return 4;
    }

    {
        let p = "abc";
        if p.chars().nth(0) != Some('a') || p.chars().nth(1) != Some('b') || p.chars().nth(2) != Some('c') || p.chars().nth(3) != Some('\0') {
            return 5;
        }
    }

    0
}