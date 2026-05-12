fn main() {
    let mut s = String::from("abc");
    let mut t: [char; 3] = ['a', 'b', 'c'];

    if s.chars().nth(0).unwrap()!= 'a' || s.chars().nth(1).unwrap()!= 'b' || s.chars().nth(2).unwrap()!= 'c' || s.chars().nth(3).unwrap()!= '\0' {
        std::process::exit(1);
    }

    if t[0]!= 'a' || t[1]!= 'b' || t[2]!= 'c' {
        std::process::exit(2);
    }

    s.clear();
    s.push('x');
    t[2] = 'y';

    if s.chars().nth(0).unwrap()!= 'x' {
        std::process::exit(3);
    }
    if t[2]!= 'y' {
        std::process::exit(4);
    }

    {
        let p = "abc";
        if p.chars().nth(0).unwrap()!= 'a' || p.chars().nth(1).unwrap()!= 'b' || p.chars().nth(2).unwrap()!= 'c' || p.chars().nth(3).unwrap()!= '\0' {
            std::process::exit(5);
        }
    }

    std::process::exit(0);
}