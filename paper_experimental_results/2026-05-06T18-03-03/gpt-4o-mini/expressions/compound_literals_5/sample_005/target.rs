fn main() {
    let s1 = "/tmp/fileXXXXXX";

    let s2 = String::from("/tmp/fileXXXXXX");
    let s2_bytes = s2.as_bytes();

    let s3 = String::from("/tmp/fileXXXXXX");
    let s3_bytes = s3.as_bytes();

    if s1.as_bytes()[0] != b'/' || s2_bytes[0] != b'/' || s3_bytes[0] != b'/' {
        std::process::exit(1);
    }
    if s1.as_bytes()[1] != b't' || s2_bytes[1] != b't' || s3_bytes[1] != b't' {
        std::process::exit(2);
    }

    let mut s2_chars: Vec<char> = s2.chars().collect();
    s2_chars[0] = 'X';
    let modified_s2: String = s2_chars.iter().collect();

    if modified_s2.as_bytes()[0] != b'X' {
        std::process::exit(3);
    }

    std::process::exit(0);
}