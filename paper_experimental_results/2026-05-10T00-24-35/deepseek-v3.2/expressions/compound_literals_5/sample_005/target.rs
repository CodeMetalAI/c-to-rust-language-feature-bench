fn main() {
    let s1 = "/tmp/fileXXXXXX";
    let s2_array = ["/tmp/fileXXXXXX"];
    let s2 = s2_array[0];
    let s3_array = ["/tmp/fileXXXXXX"];
    let s3 = s3_array[0];

    if s1.chars().next().unwrap() != '/' || s2.chars().next().unwrap() != '/' || s3.chars().next().unwrap() != '/' {
        return std::process::exit(1);
    }
    if s1.chars().nth(1).unwrap() != 't' || s2.chars().nth(1).unwrap() != 't' || s3.chars().nth(1).unwrap() != 't' {
        return std::process::exit(2);
    }

    // s2 is a slice from a fixed array, immutable in Rust. We need a mutable version.
    let mut s2_mut = s2_array[0].to_string();
    s2_mut.replace_range(0..1, "X");
    if s2_mut.chars().next().unwrap() != 'X' {
        return std::process::exit(3);
    }

    std::process::exit(0);
}