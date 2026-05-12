fn main() {
    let mut s = String::from("abc");
    let mut t = ['a', 'b', 'c'];

    if s.chars().collect::<Vec<char>>()!= vec!['a', 'b', 'c'] {
        return 1;
    }

    if t!= ['a', 'b', 'c'] {
        return 2;
    }

    s.replace_range(0..1, "x");
    t[2] = 'y';

    if s.chars().next().unwrap()!= 'x' {
        return 3;
    }
    if t[2]!= 'y' {
        return 4;
    }

    let p = "abc";
    if p.chars().collect::<Vec<char>>()!= vec!['a', 'b', 'c'] {
        return 5;
    }

    return 0;
}