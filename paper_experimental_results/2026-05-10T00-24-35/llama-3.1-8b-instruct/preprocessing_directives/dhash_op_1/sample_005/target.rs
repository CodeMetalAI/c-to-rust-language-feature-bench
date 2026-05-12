fn main() {
    let p = concat!("x", " ", "#", "#", " ", "y");
    if p.len() != 7 {
        return;
    }
    if p.as_bytes()[0] != b'x' {
        return;
    }
    if p.as_bytes()[1] != b' ' {
        return;
    }
    if p.as_bytes()[2] != b'#' {
        return;
    }
    if p.as_bytes()[3] != b'#' {
        return;
    }
    if p.as_bytes()[4] != b' ' {
        return;
    }
    if p.as_bytes()[5] != b'y' {
        return;
    }
    if p.as_bytes()[6] != b'\0' {
        return;
    }
    println!("0");
}