fn main() {
    let s1: &str = "/tmp/fileXXXXXX";

    let mut s2: String = "/tmp/fileXXXXXX".to_string();

    let s3: &str = "/tmp/fileXXXXXX";

    if s1.chars().nth(0) != Some('/') || s2.chars().nth(0) != Some('/') || s3.chars().nth(0) != Some('/') {
        std::process::exit(1);
    }
    if s1.chars().nth(1) != Some('t') || s2.chars().nth(1) != Some('t') || s3.chars().nth(1) != Some('t') {
        std::process::exit(2);
    }

    s2.replace_range(..1, "X");
    if s2.chars().nth(0) != Some('X') {
        std::process::exit(3);
    }
}