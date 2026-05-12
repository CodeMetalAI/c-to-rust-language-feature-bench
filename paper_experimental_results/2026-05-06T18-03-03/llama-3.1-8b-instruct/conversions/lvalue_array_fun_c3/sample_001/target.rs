use std::mem;

fn main() {
    let a: [i32; 3] = [10, 20, 30];

    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        return 6;
    }
    if std::mem::align_of_val(&a) != std::mem::align_of::<i32>() {
        return 7;
    }

    let s = "abc";
    if s.as_ptr() as usize != 3 {
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
    if s.as_bytes()[3] != b'\0' {
        return 12;
    }

    return 0;
}