fn main() {
    let s = "abc";
    let mut t = String::from("abc");

    if s.chars().zip(s.chars()).any(|(a, b)| a != b) {
        return;
    }

    if t.chars().zip(t.chars()).any(|(a, b)| a != b) {
        return;
    }

    s.chars().next().unwrap() = 'x';
    t.chars().next_back().unwrap() = 'y';

    if s.chars().next().unwrap() != 'x' {
        return;
    }
    if t.chars().next_back().unwrap() != 'y' {
        return;
    }

    {
        let p = "abc";
        if p.chars().zip(p.chars()).any(|(a, b)| a != b) {
            return;
        }
    }

    println!("Success");
}