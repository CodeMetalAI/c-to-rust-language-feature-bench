fn main() {
    let s1 = "/tmp/fileXXXXXX";

    let mut s2 = [b'/', b't', b'm', b'p', b'/', b'f', b'i', b'l', b'e', b'X', b'X', b'X', b'X', b'X', b'X', b'\0'].to_vec();

    let s3 = [b'/', b't', b'm', b'p', b'/', b'f', b'i', b'l', b'e', b'X', b'X', b'X', b'X', b'X', b'X', b'\0'];

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
}