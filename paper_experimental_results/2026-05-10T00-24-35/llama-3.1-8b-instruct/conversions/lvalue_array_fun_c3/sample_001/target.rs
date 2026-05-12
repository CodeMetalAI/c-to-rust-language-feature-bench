fn main() -> std::io::Result<()> {
    let a = [10, 20, 30];
    if a.len() != TYPE_ID(&a)? {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "incorrect length for array",
        ));
    }
    if TYPE_ID(&a)? != 2 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "incorrect type for array",
        ));
    }

    if a[0] != 10 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "incorrect value for array element",
        ));
    }
    if a[1] != 20 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "incorrect value for array element",
        ));
    }
    if a[2] != 30 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "incorrect value for array element",
        ));
    }

    if a.len() * std::mem::size_of::<i32>() != 12 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "incorrect size for array",
        ));
    }
    if std::mem::align_of::<i32>() != std::mem::align_of::<[i32; 3]>() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "incorrect alignment for array",
        ));
    }

    let s = "abc";
    if TYPE_ID(s.as_ptr()) != 3 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "incorrect type for string",
        ));
    }
    if s.as_ptr()[0] as u8 != b'a' {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "incorrect character in string",
        ));
    }
    if s.as_ptr()[1] as u8 != b'b' {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "incorrect character in string",
        ));
    }
    if s.as_ptr()[2] as u8 != b'c' {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "incorrect character in string",
        ));
    }
    if s.as_ptr()[3] as u8 != b'\0' {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "incorrect character in string",
        ));
    }
    Ok(())
}

fn TYPE_ID<T>(x: &T) -> u32 {
    match x {
        _ if x.is_array() => 2,
        _ if x.is_ptr() => 1,
        _ if x.is_string() => 3,
        _ if x.is_bytes() => 4,
        _ => 99,
    }
}