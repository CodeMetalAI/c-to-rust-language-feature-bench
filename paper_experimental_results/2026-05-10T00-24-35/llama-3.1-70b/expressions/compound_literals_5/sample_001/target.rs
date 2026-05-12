fn main() {
    let s1: &str = "/tmp/fileXXXXXX";
    let s2: String = "/tmp/fileXXXXXX".to_string();
    let s3: &str = "/tmp/fileXXXXXX";

    if s1.chars().next().unwrap()!= '/' || s2.chars().next().unwrap()!= '/' || s3.chars().next().unwrap()!= '/' {
        std::process::exit(1);
    }
    if s1.chars().nth(1).unwrap()!= 't' || s2.chars().nth(1).unwrap()!= 't' || s3.chars().nth(1).unwrap()!= 't' {
        std::process::exit(2);
    }

    let mut s2_chars: Vec<char> = s2.chars().collect();
    s2_chars[0] = 'X';
    if s2_chars[0]!= 'X' {
        std::process::exit(3);
    }

    std::process::exit(0);
}