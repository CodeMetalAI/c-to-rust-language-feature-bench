fn main() {
    let a = [10, 20, 30];

    if !std::any::type_name::<&[i32]>().starts_with("&[i32]")
        || !std::any::type_name::<&[i32; 3]>().starts_with("&[i32; 3]")
    {
        return 1;
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
    if std::mem::align_of::<[i32; 3]>() != std::mem::align_of::<i32>() {
        return 7;
    }

    let s = "abc".as_bytes();
    if s[0] != b'a' {
        return 9;
    }
    if s[1] != b'b' {
        return 10;
    }
    if s[2] != b'c' {
        return 11;
    }
    if s.len() != 3 {
        return 12;
    }

    return 0;
}