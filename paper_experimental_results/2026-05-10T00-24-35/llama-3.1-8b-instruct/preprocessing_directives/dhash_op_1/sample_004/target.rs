fn main() -> std::io::Result<()> {
    let p = format!("x {} # # {} ", 'x', 'y');
    if p.len() != 7 {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Size mismatch"));
    }
    if p.chars().nth(0).unwrap() != 'x' {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Char mismatch at index 0"));
    }
    if p.chars().nth(1).unwrap() != ' ' {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Char mismatch at index 1"));
    }
    if p.chars().nth(2).unwrap() != '#' {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Char mismatch at index 2"));
    }
    if p.chars().nth(3).unwrap() != '#' {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Char mismatch at index 3"));
    }
    if p.chars().nth(4).unwrap() != ' ' {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Char mismatch at index 4"));
    }
    if p.chars().nth(5).unwrap() != 'y' {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Char mismatch at index 5"));
    }
    if p.chars().nth(6).unwrap() != '\0' {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Char mismatch at index 6"));
    }
    Ok(())
}