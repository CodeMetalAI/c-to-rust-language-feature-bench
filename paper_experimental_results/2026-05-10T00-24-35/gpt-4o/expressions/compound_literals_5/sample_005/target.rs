fn main() -> i32 {
    const S1: &str = "/tmp/fileXXXXXX";

    let mut s2 = String::from("/tmp/fileXXXXXX");

    const S3: &str = "/tmp/fileXXXXXX";

    if S1.chars().nth(0) != Some('/') || s2.chars().nth(0) != Some('/') || S3.chars().nth(0) != Some('/') {
        return 1;
    }
    if S1.chars().nth(1) != Some('t') || s2.chars().nth(1) != Some('t') || S3.chars().nth(1) != Some('t') {
        return 2;
    }

    s2.replace_range(0..1, "X");
    if s2.chars().nth(0) != Some('X') {
        return 3;
    }

    0
}