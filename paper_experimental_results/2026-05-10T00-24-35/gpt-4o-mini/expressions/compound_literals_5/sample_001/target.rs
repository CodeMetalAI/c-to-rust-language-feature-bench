fn main() {
    let s1 = "/tmp/fileXXXXXX";

    let s2 = String::from("/tmp/fileXXXXXX");

    let s3 = String::from("/tmp/fileXXXXXX");

    if s1.chars().nth(0) != Some('/') || s2.chars().nth(0) != Some('/') || s3.chars().nth(0) != Some('/') {
        std::process::exit(1);
    }
    if s1.chars().nth(1) != Some('t') || s2.chars().nth(1) != Some('t') || s3.chars().nth(1) != Some('t') {
        std::process::exit(2);
    }

    let mut s2_chars: Vec<char> = s2.chars().collect();
    s2_chars[0] = 'X';
    let s2 = s2_chars.iter().collect::<String>();
    
    if s2.chars().nth(0) != Some('X') {
        std::process::exit(3);
    }

    std::process::exit(0);
}