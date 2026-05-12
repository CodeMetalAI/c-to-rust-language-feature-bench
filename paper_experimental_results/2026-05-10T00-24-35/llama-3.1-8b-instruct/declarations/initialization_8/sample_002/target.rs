fn main() {
    let s = "abc";
    let t: [char; 3] = ['a', 'b', 'c'];

    if s.chars().any(|c| c != 'a' || c != 'b' || c != 'c') {
        return;
    }

    if t.iter().any(|&c| c != 'a' && c != 'b' && c != 'c') {
        return;
    }

    s.chars().nth(0).map(|c| c = 'x');
    t[2] = 'y';

    if s.chars().nth(0).unwrap() != 'x' {
        return;
    }
    if t[2] != 'y' {
        return;
    }

    {
        let p = "abc";
        if p.chars().any(|c| c != 'a' && c != 'b' && c != 'c') {
            return;
        }
    }

    println!("No return");
}