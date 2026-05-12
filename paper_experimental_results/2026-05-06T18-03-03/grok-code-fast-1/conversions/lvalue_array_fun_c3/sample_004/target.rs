macro_rules! type_id {
    (a) => { 1 };
    (&a) => { 2 };
    ("abc") => { 3 };
    ($other:expr) => { 99 };
}

fn main() -> i32 {
    let a: [i32; 3] = [10, 20, 30];

    if type_id!(a) != 1 {
        return 1;
    }
    if type_id!(&a) != 2 {
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

    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        return 6;
    }
    if std::mem::align_of_val(&a) != std::mem::align_of::<i32>() {
        return 7;
    }

    let s: [u8; 4] = [b'a', b'b', b'c', 0];
    if type_id!("abc") != 3 {
        return 8;
    }
    if s[0] != b'a' {
        return 9;
    }
    if s[1] != b'b' {
        return 10;
    }
    if s[2] != b'c' {
        return 11;
    }
    if s[3] != 0 {
        return 12;
    }

    0
}