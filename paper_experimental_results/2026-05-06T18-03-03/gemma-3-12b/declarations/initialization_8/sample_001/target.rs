fn main() {
    let mut s = "abc".to_string();
    let t = ['a', 'b', 'c'];

    if s.chars().nth(0) != Some('a') || s.chars().nth(1) != Some('b') || s.chars().nth(2) != Some('c') || s.chars().nth(3) == Some('\0') {
        return 1;
    }

    if t[0] != 'a' || t[1] != 'b' || t[2] != 'c' {
        return 2;
    }

    let mut s_chars: Vec<char> = s.chars().collect();
    s_chars[0] = 'x';
    s = s_chars.into_iter().collect();

    let mut t_mut = t.to_vec();
    t_mut[2] = 'y';

    if s.chars().nth(0) != Some('x') {
        return 3;
    }
    if t_mut[2] != 'y' {
        return 4;
    }

    {
        let p = "abc";
        if p.chars().nth(0) != Some('a') || p.chars().nth(1) != Some('b') || p.chars().nth(2) != Some('c') || p.chars().nth(3) == Some('\0') {
            return 5;
        }
    }

    0
}