fn main() {
    let p: &str = "x ## y";
    if p.len() != 6 {
        std::process::exit(1);
    }
    if p.chars().nth(0).unwrap() != 'x' {
        std::process::exit(2);
    }
    if p.chars().nth(1).unwrap() != ' ' {
        std::process::exit(3);
    }
    if p.chars().nth(2).unwrap() != '#' {
        std::process::exit(4);
    }
    if p.chars().nth(3).unwrap() != '#' {
        std::process::exit(5);
    }
    if p.chars().nth(4).unwrap() != ' ' {
        std::process::exit(6);
    }
    if p.chars().nth(5).unwrap() != 'y' {
        std::process::exit(7);
    }
    std::process::exit(0);
}