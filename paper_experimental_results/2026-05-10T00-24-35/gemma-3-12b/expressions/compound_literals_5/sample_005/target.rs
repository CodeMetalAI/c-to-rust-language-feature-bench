fn main() {
    let s1 = "/tmp/fileXXXXXX";
    let s2 = String::from("/tmp/fileXXXXXX");
    let s3 = "/tmp/fileXXXXXX";

    if s1.chars().next() != Some('/') || s2.chars().next() != Some('/') || s3.chars().next() != Some('/') {
        return 1;
    }
    if s1.chars().nth(1) != Some('t') || s2.chars().nth(1) != Some('t') || s3.chars().nth(1) != Some('t') {
        return 2;
    }

    let mut s2_mut = String::from(s2);
    s2_mut.replace_range(0..1, "X");
    if s2_mut.chars().next() != Some('X') {
        return 3;
    }

    0
}