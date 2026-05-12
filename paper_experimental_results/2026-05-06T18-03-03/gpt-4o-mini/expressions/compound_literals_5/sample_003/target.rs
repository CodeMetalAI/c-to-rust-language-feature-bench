fn main() -> i32 {
    let s1 = "/tmp/fileXXXXXX";

    let mut s2 = String::from("/tmp/fileXXXXXX");
    let s3 = String::from("/tmp/fileXXXXXX");

    if s1.chars().nth(0) != Some('/') || s2.chars().nth(0) != Some('/') || s3.chars().nth(0) != Some('/') {
        return 1;
    }
    if s1.chars().nth(1) != Some('t') || s2.chars().nth(1) != Some('t') || s3.chars().nth(1) != Some('t') {
        return 2;
    }

    s2.replace_range(0..1, "X");
    if s2.chars().nth(0) != Some('X') {
        return 3;
    }

    0
}