fn main() -> i32 {
    let mut s = String::from("abc");
    let t: [&str; 1] = ["abc"];

    if s.chars().nth(0) != Some('a') || s.chars().nth(1) != Some('b') || s.chars().nth(2) != Some('c') || s.chars().nth(3) != Some('\0') {
        return 1;
    }

    if t[0].chars().nth(0) != Some('a') || t[0].chars().nth(1) != Some('b') || t[0].chars().nth(2) != Some('c') {
        return 2;
    }

    s.replace_range(0..1, "x");
    let t = t[0].replace_range(2..3, "y");

    if s.chars().nth(0) != Some('x') {
        return 3;
    }
    if t.chars().nth(2) != Some('y') {
        return 4;
    }

    {
        let p = "abc";
        if p.chars().nth(0) != Some('a') || p.chars().nth(1) != Some('b') || p.chars().nth(2) != Some('c') || p.chars().nth(3) != Some('\0') {
            return 5;
        }
    }

    return 0;
}