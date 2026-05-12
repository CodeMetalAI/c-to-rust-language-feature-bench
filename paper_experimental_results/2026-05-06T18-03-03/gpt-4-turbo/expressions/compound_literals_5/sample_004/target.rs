fn main() {
    let s1 = "/tmp/fileXXXXXX";

    let mut s2 = "/tmp/fileXXXXXX".to_owned();

    let s3 = "/tmp/fileXXXXXX";

    if s1.chars().nth(0) != Some('/') || s2.chars().nth(0) != Some('/') || s3.chars().nth(0) != Some('/') {
        std::process::exit(1);
    }
    if s1.chars().nth(1) != Some('t') || s2.chars().nth(1) != Some('t') || s3.chars().nth(1) != Some('t') {
        std::process::exit(2);
    }

    unsafe {
        let s2_bytes = s2.as_bytes_mut();
        s2_bytes[0] = b'X';
    }
    if s2.chars().nth(0) != Some('X') {
        std::process::exit(3);
    }
}