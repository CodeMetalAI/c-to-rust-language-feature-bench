fn main() {
    let s1 = "/tmp/fileXXXXXX";

    let s2 = ["/tmp/fileXXXXXX".as_bytes()].into_iter().collect::<String>();
    let s2 = s2.as_bytes();

    let s3 = ["/tmp/fileXXXXXX".as_bytes()].into_iter().collect::<String>();
    let s3 = s3.as_bytes();

    if s1.as_bytes()[0]!= b'/' || s2[0]!= b'/' || s3[0]!= b'/' {
        return;
    }
    if s1.as_bytes()[1]!= b't' || s2[1]!= b't' || s3[1]!= b't' {
        return;
    }

    s2[0] = b'X' as u8;
    if s2[0]!= b'X' as u8 {
        return;
    }

    assert_eq!(0, 0);
}