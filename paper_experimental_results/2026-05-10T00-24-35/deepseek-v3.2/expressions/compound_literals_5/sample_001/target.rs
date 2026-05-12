fn main() {
    let s1: &str = "/tmp/fileXXXXXX";
    let s2_arr = ["/tmp/fileXXXXXX"];
    let s2: &mut str = &mut s2_arr[0];
    let s3_arr = ["/tmp/fileXXXXXX"];
    let s3: &str = &s3_arr[0];

    if s1.chars().next().unwrap() != '/' || s2.chars().next().unwrap() != '/' || s3.chars().next().unwrap() != '/' {
        return std::process::exit(1);
    }
    if s1.chars().nth(1).unwrap() != 't' || s2.chars().nth(1).unwrap() != 't' || s3.chars().nth(1).unwrap() != 't' {
        return std::process::exit(2);
    }

    // Rust strings are immutable by default, but we have a mutable reference to s2's underlying array
    // We need to modify the array directly, not the string slice
    let mut s2_array = s2_arr[0].to_string();
    s2_array.replace_range(0..1, "X");
    let s2 = &mut s2_array;

    if s2.chars().next().unwrap() != 'X' {
        return std::process::exit(3);
    }

    std::process::exit(0);
}