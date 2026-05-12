fn main() {
    let s1 = "/tmp/fileXXXXXX";
    let mut s2 = *b"/tmp/fileXXXXXX\0"; // Convert to byte array with null termination
    let s3 = "/tmp/fileXXXXXX";

    // Check conditions as in the C++ code, comparing characters
    if s1.as_bytes()[0] != b'/' || s2[0] != b'/' || s3.as_bytes()[0] != b'/' {
        std::process::exit(1);
    }
    if s1.as_bytes()[1] != b't' || s2[1] != b't' || s3.as_bytes()[1] != b't' {
        std::process::exit(2);
    }

    // Mutate s2
    s2[0] = b'X';
    if s2[0] != b'X' {
        std::process::exit(3);
    }

    // Successfully exit
    std::process::exit(0);
}