fn main() {
    let a = [10, 20, 30];
    if a.len() != 3 {
        return 1;
    }
    if &a.len() != 2 {
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

    let s = "abc";
    if s.len() != 3 {
        return 8;
    }
    if s.as_bytes()[0] != b'a' as u8 {
        return 9;
    }
    if s.as_bytes()[1] != b'b' as u8 {
        return 10;
    }
    if s.as_bytes()[2] != b'c' as u8 {
        return 11;
    }
    if s.as_bytes()[3] != b'\0' as u8 {
        return 12;
    }

    0
}