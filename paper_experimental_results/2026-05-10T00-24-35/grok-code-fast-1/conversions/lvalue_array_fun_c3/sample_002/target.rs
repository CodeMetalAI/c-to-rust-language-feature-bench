macro_rules! type_id {
    ($x:expr) => {
        match stringify!($x) {
            "a" => 1,
            "&a" => 2,
            "\"abc\"" => 3,
            _ => 99,
        }
    };
}

fn main() {
    let a: [i32; 3] = [10, 20, 30];

    if type_id!(a) != 1 {
        std::process::exit(1);
    }
    if type_id!(&a) != 2 {
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

    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        std::process::exit(6);
    }
    if std::mem::align_of_val(&a) != std::mem::align_of::<i32>() {
        std::process::exit(7);
    }

    let s: [u8; 4] = [b'a', b'b', b'c', 0];
    if type_id!("abc") != 3 {
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