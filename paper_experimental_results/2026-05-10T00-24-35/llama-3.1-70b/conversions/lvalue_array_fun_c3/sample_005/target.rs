fn main() {
    let a: [i32; 3] = [10, 20, 30];

    if !std::ptr::eq(a.as_ptr(), a.as_ptr().offset(0)) {
        return 1;
    }
    if !std::ptr::eq(a.as_ptr(), a.as_ptr().offset(1)) {
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
    if std::mem::align_of::<[i32; 3]>() != std::mem::align_of::<i32>() {
        return 7;
    }

    let s: [char; 4] = ['a', 'b', 'c', '\0'];
    if "abc".as_ptr() as usize != "abc".as_ptr().offset(0) as usize {
        return 8;
    }
    if s[0] != 'a' {
        return 9;
    }
    if s[1] != 'b' {
        return 10;
    }
    if s[2] != 'c' {
        return 11;
    }
    if s[3] != '\0' {
        return 12;
    }

    return 0;
}