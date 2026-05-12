fn main() {
    let p = format!("x {} #{} {}", "", "", "y").into_bytes();

    if p.len() != 7 {
        std::process::exit(1);
    }
    if p[0] != b'x' {
        std::process::exit(2);
    }
    if p[1] != b' ' {
        std::process::exit(3);
    }
    if p[2] != b'#' {
        std::process::exit(4);
    }
    if p[3] != b'#' {
        std::process::exit(5);
    }
    if p[4] != b' ' {
        std::process::exit(6);
    }
    if p[5] != b'y' {
        std::process::exit(7);
    }
    if p[6] != b'\0' {
        std::process::exit(8);
    }
}