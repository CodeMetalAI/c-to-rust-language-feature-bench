fn main() {
    let p = "x ## y";
    if p.len() != 6 {
        return std::process::exit(1);
    }
    if p.chars().nth(0) != Some('x') {
        return std::process::exit(2);
    }
    if p.chars().nth(1) != Some(' ') {
        return std::process::exit(3);
    }
    if p.chars().nth(2) != Some('#') {
        return std::process::exit(4);
    }
    if p.chars().nth(3) != Some('#') {
        return std::process::exit(5);
    }
    if p.chars().nth(4) != Some(' ') {
        return std::process::exit(6);
    }
    if p.chars().nth(5) != Some('y') {
        return std::process::exit(7);
    }
    if p.len() != 6 || p.chars().nth(6).is_some() {
        return std::process::exit(8);
    }
    std::process::exit(0);
}