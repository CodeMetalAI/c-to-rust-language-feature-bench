fn main() {
    let s1 = "/tmp/fileXXXXXX";
    let mut s2_array = ['\0'; 17];
    let s2_slice = "/tmp/fileXXXXXX";
    s2_array.copy_from_slice(s2_slice.chars().collect::<Vec<char>>().as_slice());
    let mut s2 = s2_array.as_mut_slice();
    let s3_array = ['/','t','m','p','/','f','i','l','e','X','X','X','X','X','X','\0'];
    let s3 = &s3_array[..];

    if s1.chars().next().unwrap() != '/' || s2[0] != '/' || s3[0] != '/' {
        std::process::exit(1);
    }
    if s1.chars().nth(1).unwrap() != 't' || s2[1] != 't' || s3[1] != 't' {
        std::process::exit(2);
    }

    s2[0] = 'X';
    if s2[0] != 'X' {
        std::process::exit(3);
    }

    std::process::exit(0);
}