fn main() {
    let s1 = "/tmp/fileXXXXXX";
    let mut s2 = String::from("/tmp/fileXXXXXX");
    let s3 = "/tmp/fileXXXXXX";

    if s1.chars().next() != Some('/') || s2.chars().next() != Some('/') || s3.chars().next() != Some('/') {
        std::process::exit(1);
    }
    if s1.chars().nth(1) != Some('t') || s2.chars().nth(1) != Some('t') || s3.chars().nth(1) != Some('t') {
        std::process::exit(2);
    }

    s2.replace_range(0..1, "X");
    if s2.chars().next() != Some('X') {
        std::process::exit(3);
    }

    std::process::exit(0);
}