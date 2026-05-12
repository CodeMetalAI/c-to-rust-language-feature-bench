fn main() {
    let p = "x ## y";
    if p.len() + 1 != 7 {
        std::process::exit(1);
    }
    let chars: Vec<char> = p.chars().collect();
    if chars[0] != 'x' {
        std::process::exit(2);
    }
    if chars[1] != ' ' {
        std::process::exit(3);
    }
    if chars[2] != '#' {
        std::process::exit(4);
    }
    if chars[3] != '#' {
        std::process::exit(5);
    }
    if chars[4] != ' ' {
        std::process::exit(6);
    }
    if chars[5] != 'y' {
        std::process::exit(7);
    }
    if p.as_bytes()[6] != b'\0' {
        std::process::exit(8);
    }
    std::process::exit(0);
}