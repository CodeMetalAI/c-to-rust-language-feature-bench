fn main() {
    let p = "x # # y\0";

    if p.len() != 7 {
        std::process::exit(1);
    }
    if p.chars().nth(0) != Some('x') {
        std::process::exit(2);
    }
    if p.chars().nth(1) != Some(' ') {
        std::process::exit(3);
    }
    if p.chars().nth(2) != Some('#') {
        std::process::exit(4);
    }
    if p.chars().nth(3) != Some('#') {
        std::process::exit(5);
    }
    if p.chars().nth(4) != Some(' ') {
        std::process::exit(6);
    }
    if p.chars().nth(5) != Some('y') {
        std::process::exit(7);
    }
    if p.chars().nth(6) != Some('\0') {
        std::process::exit(8);
    }
    std::process::exit(0);
}