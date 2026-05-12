use std::process::exit;

fn main() {
    let p = b"x ## y\0";
    if p.len() != 7 {
        exit(1);
    }
    if p[0] != b'x' {
        exit(2);
    }
    if p[1] != b' ' {
        exit(3);
    }
    if p[2] != b'#' {
        exit(4);
    }
    if p[3] != b'#' {
        exit(5);
    }
    if p[4] != b' ' {
        exit(6);
    }
    if p[5] != b'y' {
        exit(7);
    }
    if p[6] != b'\0' {
        exit(8);
    }
    exit(0);
}