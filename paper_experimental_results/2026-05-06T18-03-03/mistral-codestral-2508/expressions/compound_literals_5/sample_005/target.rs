fn main() {
    let s1 = "/tmp/fileXXXXXX";

    let mut s2 = [b'/', b't', b'm', b'p', b'/', b'f', b'i', b'l', b'e', b'X', b'X', b'X', b'X', b'X', b'X'];
    let s2_str = std::str::from_utf8(&s2).unwrap();

    let s3 = [b'/', b't', b'm', b'p', b'/', b'f', b'i', b'l', b'e', b'X', b'X', b'X', b'X', b'X', b'X'];
    let s3_str = std::str::from_utf8(&s3).unwrap();

    if s1.chars().nth(0) != Some('/') || s2_str.chars().nth(0) != Some('/') || s3_str.chars().nth(0) != Some('/') {
        std::process::exit(1);
    }
    if s1.chars().nth(1) != Some('t') || s2_str.chars().nth(1) != Some('t') || s3_str.chars().nth(1) != Some('t') {
        std::process::exit(2);
    }

    s2[0] = b'X';
    if s2[0] != b'X' {
        std::process::exit(3);
    }
}