fn main() {
    let a = [10, 20, 30];

    if TYPE_ID(&a) != 2 {
        return 1;
    }
    if TYPE_ID(&a[0]) != 1 {
        return 2;
    }

    if a[0] != 10 {
        return 3;
    }
    if a[1] != 20 {
        return 4;
    }
    if a[2] != 30 {
        return 5;
    }

    if std::mem::size_of::<[i32; 3]>() != 3 * std::mem::size_of::<i32>() {
        return 6;
    }
    if std::mem::align_of::<[i32; 3]>() != std::mem::align_of::<i32>() {
        return 7;
    }

    let s = "abc";
    if TYPE_ID(s) != 3 {
        return 8;
    }
    if s.as_bytes()[0] != b'a' {
        return 9;
    }
    if s.as_bytes()[1] != b'b' {
        return 10;
    }
    if s.as_bytes()[2] != b'c' {
        return 11;
    }
    if s.as_bytes()[3] != 0 {
        return 12;
    }

    return 0;
}

#[allow(dead_code)]
macro_rules! TYPE_ID {
    ($x:expr) => {
        match $x {
            _ if std::ptr::addr_of!($x) as *const _ == std::ptr::addr_of!(&a[0]) as *const _ => 1,
            _ if std::ptr::addr_of!($x) as *const _ == std::ptr::addr_of!(&a) as *const _ => 2,
            _ if $x == "abc" => 3,
            _ if $x == "abc" => 4,
            _ => 99,
        }
    };
}