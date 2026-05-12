fn main() {
    let s1 = "/tmp/fileXXXXXX";
    let s2 = ["/tmp/fileXXXXXX".as_bytes()[0]; 12];
    let mut s3 = ["/tmp/fileXXXXXX".as_bytes()[0]; 12].to_vec();

    if s1.as_bytes()[0]!= b'/' || s2[0]!= b'/' || s3[0]!= b'/' {
        return;
    }
    if s1.as_bytes()[1]!= b't' || s2[1]!= b't' || s3[1]!= b't' {
        std::process::exit(2);
    }

    s3[0] = b'X';
    if s3[0]!= b'X' {
        std::process::exit(3);
    }

    std::process::exit(0);
}