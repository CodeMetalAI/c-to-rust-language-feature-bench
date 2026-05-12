fn main() {
    let p = "x ## y";

    if p.len() != 6 {
        std::process::exit(1);
    }
    if p.as_bytes()[0] != b'x' {
        std::process::exit(2);
    }
    if p.as_bytes()[1] != b' ' {
        std::process::exit(3);
    }
    if p.as_bytes()[2] != b'#' {
        std::process::exit(4);
    }
    if p.as_bytes()[3] != b'#' {
        std::process::exit(5);
    }
    if p.as_bytes()[4] != b' ' {
        std::process::exit(6);
    }
    if p.as_bytes()[5] != b'y' {
        std::process::exit(7);
    }
    if p.as_bytes().get(6).is_some() {
        std::process::exit(8);
    }
}