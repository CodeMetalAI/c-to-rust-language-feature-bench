fn main() {
    let s1 = b"/tmp/fileXXXXXX";
    let mut s2_bytes: [u8; 14] = *b"/tmp/fileXXXXXX";
    let s3 = b"/tmp/fileXXXXXX";

    if s1[0] != b'/' || s2_bytes[0] != b'/' || s3[0] != b'/' {
        std::process::exit(1);
    }
    if s1[1] != b't' || s2_bytes[1] != b't' || s3[1] != b't' {
        std::process::exit(2);
    }

    s2_bytes[0] = b'X';
    if s2_bytes[0] != b'X' {
        std::process::exit(3);
    }

    std::process::exit(0);
}