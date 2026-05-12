const P: &str = "x # # y";

fn main() -> i32 {
    if P.len() != 7 {
        return 1;
    }
    if P.as_bytes()[0] != b'x' {
        return 2;
    }
    if P.as_bytes()[1] != b' ' {
        return 3;
    }
    if P.as_bytes()[2] != b'#' {
        return 4;
    }
    if P.as_bytes()[3] != b'#' {
        return 5;
    }
    if P.as_bytes()[4] != b' ' {
        return 6;
    }
    if P.as_bytes()[5] != b'y' {
        return 7;
    }
    if P.as_bytes()[6] != b'\0' {
        return 8;
    }
    return 0;
}