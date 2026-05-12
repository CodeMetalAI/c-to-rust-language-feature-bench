fn main() {
    let p: [u8; 7] = *b"x ## y\0";

    let code = if p.len() != 7 {
        1
    } else if p[0] != b'x' {
        2
    } else if p[1] != b' ' {
        3
    } else if p[2] != b'#' {
        4
    } else if p[3] != b'#' {
        5
    } else if p[4] != b' ' {
        6
    } else if p[5] != b'y' {
        7
    } else if p[6] != b'\0' {
        8
    } else {
        0
    };

    std::process::exit(code);
}