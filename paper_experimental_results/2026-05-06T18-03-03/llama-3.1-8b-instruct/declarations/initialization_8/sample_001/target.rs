fn main() {
    let s = String::from("abc");
    let t: [char; 3] = ['a', 'b', 'c'];

    if s.chars().nth(0).unwrap()!= 'a' || s.chars().nth(1).unwrap()!= 'b' || s.chars().nth(2).unwrap()!= 'c' || s.chars().nth(3).unwrap()!= '\0' {
        return;
    }

    if t[0]!= 'a' || t[1]!= 'b' || t[2]!= 'c' {
        return;
    }

    s.push('x');
    t[2] = 'y';

    if s.chars().nth(0).unwrap()!= 'x' {
        return;
    }
    if t[2]!= 'y' {
        return;
    }

    {
        let p = String::from("abc");
        if p.chars().nth(0).unwrap()!= 'a' || p.chars().nth(1).unwrap()!= 'b' || p.chars().nth(2).unwrap()!= 'c' || p.chars().nth(3).unwrap()!= '\0' {
            return;
        }
    }

    println!("All tests passed.");
}