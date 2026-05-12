fn main() {
    let a = [10, 20, 30];

    if TYPE_ID(&a) != 2 {
        std::process::exit(1);
    }
    if TYPE_ID(&a[0] as *const i32) != 1 {
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

    if std::mem::size_of(&a) != 3 * std::mem::size_of(i32) {
        std::process::exit(6);
    }
    if std::mem::align_of(&a) != std::mem::align_of(i32) {
        std::process::exit(7);
    }

    let s = "abc".as_bytes();
    if TYPE_ID(s as *const u8) != 3 {
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

#[allow(dead_code)]
macro_rules! TYPE_ID {
    ($x:expr) => {
        match $x {
            ptr@&i32 => 1,
            ptr@&[i32; 3] => 2,
            ptr@&[u8; 4] => 3,
            _ => 99,
        }
    };
}