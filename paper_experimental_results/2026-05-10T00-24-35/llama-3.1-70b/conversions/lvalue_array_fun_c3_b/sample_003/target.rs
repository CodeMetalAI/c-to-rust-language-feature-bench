fn type_id<T>(_: &T) -> u8 {
    std::any::TypeId::of::<T>().as_u64() as u8
}

fn main() {
    let a = [10, 20, 30];

    if type_id(&a)!= type_id(&[0u32; 3]) {
        return 1;
    }
    if type_id(&a)!= type_id(&[0u32; 3]) {
        return 2;
    }

    if a[0]!= 10 {
        return 3;
    }
    if a[1]!= 20 {
        return 4;
    }
    if a[2]!= 30 {
        return 5;
    }

    if std::mem::size_of_val(&a)!= 3 * std::mem::size_of::<i32>() {
        return 6;
    }
    if std::mem::align_of::<[i32; 3]>()!= std::mem::align_of::<i32>() {
        return 7;
    }

    let s = [b'a', b'b', b'c', b'\0'];
    if type_id(&"abc")!= type_id(&[b'\0' as char]) {
        return 8;
    }
    if s[0] as char!= 'a' {
        return 9;
    }
    if s[1] as char!= 'b' {
        return 10;
    }
    if s[2] as char!= 'c' {
        return 11;
    }
    if s[3] as char!= '\0' {
        return 12;
    }

    return;
}