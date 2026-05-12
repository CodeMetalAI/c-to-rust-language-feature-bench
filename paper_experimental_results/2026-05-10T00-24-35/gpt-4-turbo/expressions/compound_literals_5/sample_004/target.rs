fn main() {
    let s1: &str = "/tmp/fileXXXXXX";

    let mut s2: Vec<char> = "/tmp/fileXXXXXX".chars().collect();

    let s3: &[char] = &"/tmp/fileXXXXXX".chars().collect::<Vec<char>>();

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

    std::process::exit(0);
}