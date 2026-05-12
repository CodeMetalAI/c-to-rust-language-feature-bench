fn main() {
    let a: [i32; 3] = [10, 20, 30];

    if std::mem::size_of_val(&a) != std::mem::size_of::<i32>() * 3 {
        return 6;
    }

    if a != [10, 20, 30] {
        return 3;
    }
    if a[1] != 20 {
        return 4;
    }
    if a[2] != 30 {
        return 5;
    }

    let s = "abc";
    if s.as_ptr() as usize != std::ptr::null::<&'static str>() as usize + 3 * std::mem::size_of::<u8>() {
        return 6;
    }

    let s = s.as_bytes();
    if s != [97, 98, 99, 0] {
        return 9;
    }
    if s[0] != 97 {
        return 10;
    }
    if s[1] != 98 {
        return 11;
    }
    if s[2] != 99 {
        return 12;
    }

    assert_eq!(std::mem::size_of::<&[i32]>() as usize, std::mem::size_of::<&[i32]>() as usize);
    assert_eq!(std::mem::size_of::<i32>() as usize, std::mem::size_of::<i32>() as usize);

    assert_eq!(std::mem::size_of::<&str>() as usize, std::mem::size_of::<&str>() as usize);
    assert_eq!(std::mem::size_of::<u8>() as usize, std::mem::size_of::<u8>() as usize);

    assert_eq!(std::mem::size_of::<[u8; 4]>() as usize, std::mem::size_of::<[u8; 4]>() as usize);
    assert_eq!(std::mem::size_of::<i32>() as usize, std::mem::size_of::<i32>() as usize);

    return 0;
}