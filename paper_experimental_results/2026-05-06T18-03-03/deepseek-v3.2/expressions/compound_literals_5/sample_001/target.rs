fn main() {
    let s1 = "/tmp/fileXXXXXX";

    let mut s2_array = ["/tmp/fileXXXXXX"];
    let s2 = s2_array[0].as_mut_ptr() as *mut char;
    let s2_slice = unsafe { std::slice::from_raw_parts_mut(s2, s1.len()) };
    // Initialize mutable string with same content
    let mut s2_vec: Vec<char> = s1.chars().collect();
    let s2 = s2_vec.as_mut_slice();

    let s3_array = ["/tmp/fileXXXXXX"];
    let s3 = s3_array[0].as_ptr() as *const char;
    let s3_slice = unsafe { std::slice::from_raw_parts(s3, s1.len()) };
    // Initialize immutable string with same content
    let s3_vec: Vec<char> = s1.chars().collect();
    let s3 = s3_vec.as_slice();

    // Convert s1 to chars for indexing
    let s1_chars: Vec<char> = s1.chars().collect();
    
    if s1_chars[0] != '/' || s2[0] != '/' || s3[0] != '/' {
        std::process::exit(1);
    }
    if s1_chars[1] != 't' || s2[1] != 't' || s3[1] != 't' {
        std::process::exit(2);
    }

    s2[0] = 'X';
    if s2[0] != 'X' {
        std::process::exit(3);
    }

    std::process::exit(0);
}