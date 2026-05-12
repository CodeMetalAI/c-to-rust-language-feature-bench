fn main() {
    let s1 = "/tmp/fileXXXXXX";

    let mut s2 = *"/tmp/fileXXXXXX";

    let s3 = "/tmp/fileXXXXXX";

    if s1.as_bytes()[0] != b'/' || s2.as_bytes()[0] != b'/' || s3.as_bytes()[0] != b'/' {
        std::process::exit(1);
    }
    if s1.as_bytes()[1] != b't' || s2.as_bytes()[1] != b't' || s3.as_bytes()[1] != b't' {
        std::process::exit(2);
    }

    s2.as_bytes_mut()[0] = b'X';
    if s2.as_bytes()[0] != b'X' {
        std::process::exit(3);
    }
}