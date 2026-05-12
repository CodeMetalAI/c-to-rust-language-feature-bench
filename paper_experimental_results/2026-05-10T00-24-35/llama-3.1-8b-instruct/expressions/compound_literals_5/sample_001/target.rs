fn main() {
    let s1 = "/tmp/fileXXXXXX";

    let s2 = ["/tmp/fileXXXXXX".as_bytes()].to_vec();

    let s3 = ["/tmp/fileXXXXXX".as_bytes()].to_vec();

    if s1.as_bytes()[0] != b'/' || s2[0] != b'/' || s3[0] != b'/' {
        return;
    }
    if s1.as_bytes()[1] != b't' || s2[1] != b't' || s3[1] != b't' {
        return;
    }

    let mut s2_bytes = s2.clone();
    s2_bytes[0] = b'X';

    if s2_bytes[0] != b'X' {
        return;
    }

    ()
}