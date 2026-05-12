fn main() {
    let s1: &str = "/tmp/fileXXXXXX";
    
    let mut s2_storage: Vec<char> = "/tmp/fileXXXXXX".chars().collect();
    
    let s3: &str = "/tmp/fileXXXXXX";
    
    if s1.chars().nth(0) != Some('/') || s2_storage[0] != '/' || s3.chars().nth(0) != Some('/') {
        std::process::exit(1);
    }
    if s1.chars().nth(1) != Some('t') || s2_storage[1] != 't' || s3.chars().nth(1) != Some('t') {
        std::process::exit(2);
    }
    
    s2_storage[0] = 'X';
    if s2_storage[0] != 'X' {
        std::process::exit(3);
    }
    
    std::process::exit(0);
}