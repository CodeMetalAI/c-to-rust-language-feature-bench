fn main() -> std::io::Result<()> {
    let s1: &str = "/tmp/fileXXXXXX";

    let s2: [&'static str; 1] = ["/tmp/fileXXXXXX"];

    let s3: [&'static str; 1] = ["/tmp/fileXXXXXX"];

    if s1.chars().nth(0).unwrap() != '/' || s2[0] != '/' || s3[0] != '/' {
        return Ok(std::process::exit(1));
    }
    if s1.chars().nth(1).unwrap() != 't' || s2[0] != 't' || s3[0] != 't' {
        return Ok(std::process::exit(2));
    }

    // Rust does not allow modification of a string literal.
    // We can create a new string and modify it:
    let mut s2 = String::from("/tmp/fileXXXXXX");
    s2.insert(0, 'X');
    if s2[0] != 'X' {
        return Ok(std::process::exit(3));
    }

    Ok(())
}