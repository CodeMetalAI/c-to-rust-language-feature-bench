trait TypeId {
    fn type_id(&self) -> i32;
}

impl TypeId for [i32; 3] {
    fn type_id(&self) -> i32 {
        1
    }
}

impl TypeId for &[i32; 3] {
    fn type_id(&self) -> i32 {
        2
    }
}

impl TypeId for &str {
    fn type_id(&self) -> i32 {
        3
    }
}

fn main() {
    let mut a: [i32; 3] = [10, 20, 30];

    if a.type_id() != 1 {
        std::process::exit(1);
    }
    if (&a).type_id() != 2 {
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

    let s: [u8; 4] = [b'a', b'b', b'c', b'\0'];
    if "abc".type_id() != 3 {
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
    if s[3] != b'\0' {
        std::process::exit(12);
    }

    std::process::exit(0);
}