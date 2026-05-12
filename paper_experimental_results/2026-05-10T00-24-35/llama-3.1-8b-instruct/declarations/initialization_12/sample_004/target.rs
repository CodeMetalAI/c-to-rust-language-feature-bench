const MAX: usize = 12;

fn main() -> std::io::Result<()> {
    let mut a = [0; MAX];

    a[0] = 1;
    a[1] = 3;
    a[2] = 5;
    a[3] = 7;
    a[4] = 9;
    a[MAX - 5] = 8;
    a[5] = 6;
    a[6] = 4;
    a[7] = 2;
    a[8] = 0;

    if a[0] != 1 {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Test 1 failed"));
    }
    if a[1] != 3 {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Test 2 failed"));
    }
    if a[2] != 5 {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Test 3 failed"));
    }
    if a[3] != 7 {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Test 4 failed"));
    }
    if a[4] != 9 {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Test 5 failed"));
    }

    if a[MAX - 5] != 8 {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Test 6 failed"));
    }
    if a[MAX - 4] != 6 {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Test 7 failed"));
    }
    if a[MAX - 3] != 4 {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Test 8 failed"));
    }
    if a[MAX - 2] != 2 {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Test 9 failed"));
    }
    if a[MAX - 1] != 0 {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Test 10 failed"));
    }

    let mut i = 5;
    while i < MAX - 5 {
        if a[i] != 0 {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Test 11 failed"));
        }
        i += 1;
    }

    Ok(())
}