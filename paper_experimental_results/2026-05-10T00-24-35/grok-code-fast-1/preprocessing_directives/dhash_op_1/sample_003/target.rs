fn main() -> i32 {
    const P: &[u8; 7] = b"x ## y\0";
    if P.len() != 7 {
        return 1;
    }
    if P[0] != b'x' {
        return 2;
    }
    if P[1] != b' ' {
        return 3;
    }
    if P[2] != b'#' {
        return 4;
    }
    if P[3] != b'#' {
        return 5;
    }
    if P[4] != b' ' {
        return 6;
    }
    if P[5] != b'y' {
        return 7;
    }
    if P[6] != 0 {
        return 8;
    }
    0
}