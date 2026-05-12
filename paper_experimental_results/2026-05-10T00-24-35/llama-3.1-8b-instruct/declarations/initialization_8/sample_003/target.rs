fn main() {
    let s = String::from("abc");
    let t = String::from("abc");

    if s.chars().any(|c| c!= 'a') || s.chars().any(|c| c!= 'b') || s.chars().any(|c| c!= 'c') {
        return;
    }

    if t.chars().any(|c| c!= 'a') || t.chars().any(|c| c!= 'b') || t.chars().any(|c| c!= 'c') {
        return;
    }

    s.push('x');
    t.push('y');

    if s.chars().any(|c| c!= 'x') {
        return;
    }
    if t.chars().any(|c| c!= 'y') {
        return;
    }

    {
        let p = String::from("abc");
        if p.chars().any(|c| c!= 'a') || p.chars().any(|c| c!= 'b') || p.chars().any(|c| c!= 'c') {
            return;
        }
    }

    println!("All checks passed");
}