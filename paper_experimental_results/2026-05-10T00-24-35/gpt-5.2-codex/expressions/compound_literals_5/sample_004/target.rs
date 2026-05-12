use std::process::exit;

fn main() {
    let s1: &[u8] = b"/tmp/fileXXXXXX\0";
    let mut s2 = *b"/tmp/fileXXXXXX\0";
    let s3: &[u8] = b"/tmp/fileXXXXXX\0";

    if s1[0] != b'/' || s2[0] != b'/' || s3[0] != b'/' {
        exit(1);
    }
    if s1[1] != b't' || s2[1] != b't' || s3[1] != b't' {
        exit(2);
    }

    s2[0] = b'X';
    if s2[0] != b'X' {
        exit(3);
    }
}