fn main() {
    let a = [10, 20, 30];

    if TYPE_ID(&a) != 2 {
        return 1;
    }
    if TYPE_ID(&a[0] as *const i32) != 1 {
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
    if TYPE_ID(s as &str) != 3 {
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

    0
}

#[allow(dead_code)]
fn TYPE_ID<T>(x: &T) -> i32 {
    let type_id = match x {
        _ if std::ptr::addr_of_mut(x) as *mut _ == std::ptr::addr_of_mut(&x[0]) as *mut _ && std::any::TypeId::of::<[i32; 3]>() == std::any::TypeId::of::<&[i32; 3]>() => 2,
        _ if std::any::TypeId::of::<&i32>() == std::any::TypeId::of::<*const i32>() => 1,
        _ if std::any::TypeId::of::<&str>() == std::any::TypeId::of::<&[u8]>() => 3,
        _ => 99,
    };
    type_id
}