fn main() {
    let s1 = "/tmp/fileXXXXXX";
    let s2 = "/tmp/fileXXXXXX".to_owned().into_boxed_str();
    let s3 = "/tmp/fileXXXXXX".to_owned().into_boxed_str();

    if s1.chars().next().unwrap()!= '/' || s2.chars().next().unwrap()!= '/' || s3.chars().next().unwrap()!= '/' {
        std::process::exit(1);
    }
    if s1.chars().nth(1).unwrap()!= 't' || s2.chars().nth(1).unwrap()!= 't' || s3.chars().nth(1).unwrap()!= 't' {
        std::process::exit(2);
    }

    let mut s2 = s2.into_string();
    s2.replace_range(0..1, "X");
    if s2.chars().next().unwrap()!= 'X' {
        std::process::exit(3);
    }

    std::process::exit(0);
}