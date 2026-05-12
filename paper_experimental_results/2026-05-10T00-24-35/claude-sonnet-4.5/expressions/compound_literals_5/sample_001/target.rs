fn main() {
    let s1: &str = "/tmp/fileXXXXXX";

    let mut s2_array: [u8; 16] = *b"/tmp/fileXXXXXX";
    let s2: &mut [u8] = &mut s2_array;

    let s3_array: &[u8] = b"/tmp/fileXXXXXX";
    let s3: &[u8] = s3_array;

    if s1.as_bytes()[0] != b'/' || s2[0] != b'/' || s3[0] != b'/' {
        std::process::exit(1);
    }
    if s1.as_bytes()[1] != b't' || s2[1] != b't' || s3[1] != b't' {
        std::process::exit(2);
    }

    s2[0] = b'X';
    if s2[0] != b'X' {
        std::process::exit(3);
    }

    std::process::exit(0);
}