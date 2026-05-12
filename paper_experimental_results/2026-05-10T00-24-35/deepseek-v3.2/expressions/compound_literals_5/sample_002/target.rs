fn main() {
    let s1: &str = "/tmp/fileXXXXXX";

    let s2: &mut [char] = &mut ['/', 't', 'm', 'p', '/', 'f', 'i', 'l', 'e', 'X', 'X', 'X', 'X', 'X', 'X'];
    let s2_ptr = s2 as *mut [char] as *mut char;

    let s3: &[char] = &['/', 't', 'm', 'p', '/', 'f', 'i', 'l', 'e', 'X', 'X', 'X', 'X', 'X', 'X'];

    if s1.chars().next().unwrap() != '/' || s2[0] != '/' || s3[0] != '/' {
        std::process::exit(1);
    }
    if s1.chars().nth(1).unwrap() != 't' || s2[1] != 't' || s3[1] != 't' {
        std::process::exit(2);
    }

    s2[0] = 'X';
    if s2[0] != 'X' {
        std::process::exit(3);
    }

    std::process::exit(0);
}