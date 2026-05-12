fn main() {
    let a = [10, 20, 30];

    if type_id(&a) != 2 {
        std::process::exit(1);
    }
    if type_id(&(&a[..])) != 2 {
        std::process::exit(2);
    }

    if a[0] != 10 {
        std::process::exit(3);
    }
    if a[1] != 20 {
        std::process::exit(4);
    }
    if a[2] != 30 {
        std::process::exit(5);
    }

    if std::mem::size_of::<[i32; 3]>() != 3 * std::mem::size_of::<i32>() {
        std::process::exit(6);
    }
    if std::mem::align_of::<[i32; 3]>() != std::mem::align_of::<i32>() {
        std::process::exit(7);
    }

    let s = "abc".as_bytes();
    if type_id(s) != 3 {
        std::process::exit(8);
    }
    if s[0] != b'a' {
        std::process::exit(9);
    }
    if s[1] != b'b' {
        std::process::exit(10);
    }
    if s[2] != b'c' {
        std::process::exit(11);
    }
    if s[3] != 0 {
        std::process::exit(12);
    }

    std::process::exit(0);
}

fn type_id(x: &[u8]) -> i32 {
    if let Some(_) = x.as_ptr() as *const [i32] {
        1
    } else if let Some(_) = x.as_ptr() as *const [i32; 3] {
        2
    } else if let Some(_) = x.as_ptr() as *const &str {
        3
    } else {
        99
    }
}