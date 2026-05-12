fn main() {
    let mut s: [char; 4] = ['a', 'b', 'c', '\0'];
    let mut t: [char; 3] = ['a', 'b', 'c'];

    if s[0]!= 'a' || s[1]!= 'b' || s[2]!= 'c' || s[3]!= '\0' {
        std::process::exit(1);
    }

    if t[0]!= 'a' || t[1]!= 'b' || t[2]!= 'c' {
        std::process::exit(2);
    }

    s[0] = 'x';
    t[2] = 'y';

    if s[0]!= 'x' {
        std::process::exit(3);
    }
    if t[2]!= 'y' {
        std::process::exit(4);
    }

    {
        let p: &[char] = &['a', 'b', 'c', '\0'];
        if p[0]!= 'a' || p[1]!= 'b' || p[2]!= 'c' || p[3]!= '\0' {
            std::process::exit(5);
        }
    }

    std::process::exit(0);
}