fn main() {
    let a: [i32; 3] = [10, 20, 30];

    if!matches!(a, [_, _, _])
        return 1;
    if!matches!(&a, &[_, _, _])
        return 2;

    if a[0]!= 10
        return 3;
    if a[1]!= 20
        return 4;
    if a[2]!= 30
        return 5;

    if std::mem::size_of_val(&a)!= 3 * std::mem::size_of::<i32>()
        return 6;
    if std::mem::align_of_val(&a)!= std::mem::align_of::<i32>()
        return 7;

    let s: [u8; 4] = [b'a', b'b', b'c', 0];
    let t: [u8; 4] = b"abc\0";
    if s!= t
        return 8;
    if s[0]!= b'a'
        return 9;
    if s[1]!= b'b'
        return 10;
    if s[2]!= b'c'
        return 11;
    if s[3]!= 0
        return 12;

    return;
}