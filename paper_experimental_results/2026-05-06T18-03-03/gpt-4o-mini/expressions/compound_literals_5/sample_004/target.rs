fn main() -> i32 {
    let s1 = "/tmp/fileXXXXXX";

    let mut s2 = String::from("/tmp/fileXXXXXX");
    let s3 = String::from("/tmp/fileXXXXXX");

    if s1.chars().nth(0).unwrap() != '/' || s2.chars().nth(0).unwrap() != '/' || s3.chars().nth(0).unwrap() != '/' {
        return 1;
    }
    if s1.chars().nth(1).unwrap() != 't' || s2.chars().nth(1).unwrap() != 't' || s3.chars().nth(1).unwrap() != 't' {
        return 2;
    }

    s2.replace_range(0..1, "X");
    if s2.chars().nth(0).unwrap() != 'X' {
        return 3;
    }

    0
}