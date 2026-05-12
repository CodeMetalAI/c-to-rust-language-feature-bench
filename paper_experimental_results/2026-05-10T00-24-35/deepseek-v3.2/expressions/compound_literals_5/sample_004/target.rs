fn main() {
    let s1 = "/tmp/fileXXXXXX";

    let s2_array = ["/tmp/fileXXXXXX"];
    let s2 = s2_array[0] as *const _ as *mut _;
    let s2 = unsafe { &mut *s2 };
    
    let s3_array = ["/tmp/fileXXXXXX"];
    let s3 = s3_array[0];

    if s1.as_bytes()[0] != b'/' || s2.as_bytes()[0] != b'/' || s3.as_bytes()[0] != b'/' {
        std::process::exit(1);
    }
    if s1.as_bytes()[1] != b't' || s2.as_bytes()[1] != b't' || s3.as_bytes()[1] != b't' {
        std::process::exit(2);
    }

    // Create a mutable buffer to modify
    let mut s2_mut = s2.as_bytes().to_vec();
    s2_mut[0] = b'X';
    let s2_modified = unsafe { std::str::from_utf8_unchecked(&s2_mut) };
    
    if s2_modified.as_bytes()[0] != b'X' {
        std::process::exit(3);
    }

    std::process::exit(0);
}