const P: &str = "x # # y";

fn main() {
    if P.len() != 7 {
        std::process::exit(1);
    }
    if P.chars().nth(0).unwrap() != 'x' {
        std::process::exit(2);
    }
    if P.chars().nth(1).unwrap() != ' ' {
        std::process::exit(3);
    }
    if P.chars().nth(2).unwrap() != '#' {
        std::process::exit(4);
    }
    if P.chars().nth(3).unwrap() != '#' {
        std::process::exit(5);
    }
    if P.chars().nth(4).unwrap() != ' ' {
        std::process::exit(6);
    }
    if P.chars().nth(5).unwrap() != 'y' {
        std::process::exit(7);
    }
    if P.chars().nth(6).unwrap() != '\0' {
        std::process::exit(8);
    }
    std::process::exit(0);
}