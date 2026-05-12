fn main() {
    let s1 = "/tmp/fileXXXXXX";
    let mut s2: Vec<char> = "/tmp/fileXXXXXX".chars().collect();
    let s3: Vec<char> = "/tmp/fileXXXXXX".chars().collect();

    if s1.chars().nth(0) != Some('/') || s2[0] != '/' || s3[0] != '/' {
        std::process::exit(1);
    }
    if s1.chars().nth(1) != Some('t') || s2[1] != 't' || s3[1] != 't' {
        std::process::exit(2);
    }

    s2[0] = 'X';
    if s2[0] != 'X' {
        std::process::exit(3);
    }
}