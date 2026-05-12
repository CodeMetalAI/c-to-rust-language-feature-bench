fn main() {
    const P: &[u8; 7] = b"x ## y\0";

    if P.len() != 7 {
        std::process::exit(1);
    }
    if P[0] != b'x' {
        std::process::exit(2);
    }
    if P[1] != b' ' {
        std::process::exit(3);
    }
    if P[2] != b'#' {
        std::process::exit(4);
    }
    if P[3] != b'#' {
        std::process::exit(5);
    }
    if P[4] != b' ' {
        std::process::exit(6);
    }
    if P[5] != b'y' {
        std::process::exit(7);
    }
    if P[6] != b'\0' {
        std::process::exit(8);
    }
}