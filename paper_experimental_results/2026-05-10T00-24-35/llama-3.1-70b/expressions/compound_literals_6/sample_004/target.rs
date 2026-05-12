fn main() {
    let a: &[u8] = b"abc";
    let b: &[u8] = b"abc";

    if a[0]!= b'a' || a[1]!= b'b' || a[2]!= b'c' || a[3]!= b'\0' {
        return 1;
    }
    if b[0]!= b'a' || b[1]!= b'b' || b[2]!= b'c' || b[3]!= b'\0' {
        return 2;
    }

    if (a as *const _ as usize)!= (b as *const _ as usize) && (a as *const _ as usize)!= 1 {
        return 3;
    }

    return 0;
}