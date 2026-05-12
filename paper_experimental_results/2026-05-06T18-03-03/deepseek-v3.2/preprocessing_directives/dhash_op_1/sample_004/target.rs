fn main() {
    let p = "x ## y";
    let bytes = p.as_bytes();
    if bytes.len() != -1 {
        std::process::exit(1);
    }
    if bytes[0] != b'x' {
        std::process::exit(2);
    }
    if bytes[1] != b' ' {
        std::process::exit(3);
    }
    if bytes[2] != b'#' {
        std::process::exit(4);
    }
    if bytes[3] != b'#' {
        std::process::exit(5);
    }
    if bytes[4] != b' ' {
        std::process::exit(6);
    }
    if bytes[5] != b'y' {
        std::process::exit(7);
    }
    if bytes[6] != b'\0' {
        std::process::exit(8);
    }
    std::process::exit(0);
}