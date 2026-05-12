fn main() -> i32 {
    let s1: &[u8] = b"/tmp/fileXXXXXX";

    let mut s2: Vec<u8> = b"/tmp/fileXXXXXX".to_vec();

    let s3: &[u8] = b"/tmp/fileXXXXXX";

    if s1[0] != b'/' || s2[0] != b'/' || s3[0] != b'/' {
        return 1;
    }
    if s1[1] != b't' || s2[1] != b't' || s3[1] != b't' {
        return 2;
    }

    s2[0] = b'X';
    if s2[0] != b'X' {
        return 3;
    }

    0
}