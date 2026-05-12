fn main() {
    let s1 = "/tmp/fileXXXXXX";
    let s2 = ["/tmp/fileXXXXXX"][..];
    let s3 = ["/tmp/fileXXXXXX"][..];

    if s1.chars().next().unwrap()!= '/' || s2.chars().next().unwrap()!= '/' || s3.chars().next().unwrap()!= '/' {
        return 1;
    }
    if s1.chars().nth(1).unwrap()!= 't' || s2.chars().nth(1).unwrap()!= 't' || s3.chars().nth(1).unwrap()!= 't' {
        return 2;
    }

    s2[0] = 'X';
    if s2[0]!= 'X' {
        return 3;
    }

    println!("0");
}