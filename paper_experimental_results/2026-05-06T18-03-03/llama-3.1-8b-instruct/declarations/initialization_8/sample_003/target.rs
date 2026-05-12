fn main() {
    let s = String::from("abc");
    let t: [char; 3] = ['a', 'b', 'c'];

    if s.chars().next() != Some('a') || s.chars().nth(1) != Some('b') || s.chars().nth(2) != Some('c') {
        return;
    }

    if t[0] != 'a' || t[1] != 'b' || t[2] != 'c' {
        return;
    }

    s.push('x');
    t[2] = 'y';

    if s.chars().next() != Some('x') {
        return;
    }
    if t[2] != 'y' {
        return;
    }

    {
        let p = String::from("abc");
        if p.chars().next() != Some('a') || p.chars().nth(1) != Some('b') || p.chars().nth(2) != Some('c') {
            return;
        }
    }

    println!("All tests passed");
}