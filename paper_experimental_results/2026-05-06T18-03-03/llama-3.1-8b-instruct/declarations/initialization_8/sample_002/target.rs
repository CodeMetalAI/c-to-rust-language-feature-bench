fn main() {
    let s = String::from("abc");
    let mut t = String::from("abc");

    if s.chars().next().unwrap() != 'a' || s.chars().nth(1).unwrap() != 'b' || s.chars().nth(2).unwrap() != 'c' || s.chars().next_back().unwrap() != '\0' {
        return;
    }

    if t.chars().next().unwrap() != 'a' || t.chars().nth(1).unwrap() != 'b' || t.chars().nth(2).unwrap() != 'c' {
        return;
    }

    s.remove(0);
    t.remove(2);

    if s.chars().next().unwrap() != 'x' {
        return;
    }
    if t.chars().nth(2).unwrap() != 'y' {
        return;
    }

    {
        let p = String::from("abc");
        if p.chars().next().unwrap() != 'a' || p.chars().nth(1).unwrap() != 'b' || p.chars().nth(2).unwrap() != 'c' || p.chars().next_back().unwrap() != '\0' {
            return;
        }
    }

    println!("All tests passed.");
}