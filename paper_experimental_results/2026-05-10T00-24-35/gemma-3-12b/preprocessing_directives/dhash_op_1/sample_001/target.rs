fn main() {
    let p = "x # # y";

    if p.len() != 7 {
        std::process::exit(1);
    }
    if p.as_bytes()[0] as char != 'x' {
        std::process::exit(2);
    }
    if p.as_bytes()[1] as char != ' ' {
        std::process::exit(3);
    }
    if p.as_bytes()[2] as char != '#' {
        std::process::exit(4);
    }
    if p.as_bytes()[3] as char != '#' {
        std::process::exit(5);
    }
    if p.as_bytes()[4] as char != ' ' {
        std::process::exit(6);
    }
    if p.as_bytes()[5] as char != 'y' {
        std::process::exit(7);
    }
    if p.as_bytes()[6] as char != '\0' {
        std::process::exit(8);
    }

    std::process::exit(0);
}