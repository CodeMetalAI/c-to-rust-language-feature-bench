fn main() {
    let a = [10, 20, 30];

    if a.len() != 3 {
        std::process::exit(1);
    }

    if a[0] != 10 {
        std::process::exit(2);
    }
    if a[1] != 20 {
        std::process::exit(3);
    }
    if a[2] != 30 {
        std::process::exit(4);
    }

    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        std::process::exit(5);
    }
    if std::mem::align_of_val(&a) != std::mem::align_of::<i32>() {
        std::process::exit(6);
    }

    let s = "abc";
    if s.len() != 3 {
        std::process::exit(7);
    }
    if s.as_bytes()[0] != b'a' {
        std::process::exit(8);
    }
    if s.as_bytes()[1] != b'b' {
        std::process::exit(9);
    }
    if s.as_bytes()[2] != b'c' {
        std::process::exit(10);
    }
    if s.as_bytes().get(3) != Some(&b'\0') {
        std::process::exit(11);
    }

    std::process::exit(0);
}