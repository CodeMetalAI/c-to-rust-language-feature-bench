fn main() {
    let s1 = "/tmp/fileXXXXXX";

    let s2_array = ["/tmp/fileXXXXXX"];
    let s2 = s2_array[0];

    let s3_array = ["/tmp/fileXXXXXX"];
    let s3 = s3_array[0];

    if s1.chars().next() != Some('/') || s2.chars().next() != Some('/') || s3.chars().next() != Some('/') {
        return std::process::exit(1);
    }
    if s1.chars().nth(1) != Some('t') || s2.chars().nth(1) != Some('t') || s3.chars().nth(1) != Some('t') {
        return std::process::exit(2);
    }

    // In Rust, strings are immutable, so we cannot modify s2 directly.
    // Instead, we create a mutable copy and check it.
    let mut s2_mut = s2.to_string();
    s2_mut.replace_range(0..1, "X");
    if s2_mut.chars().next() != Some('X') {
        return std::process::exit(3);
    }

    std::process::exit(0);
}