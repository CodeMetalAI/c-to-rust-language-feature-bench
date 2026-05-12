fn main() {
    let mut a = [10, 20, 30];

    if type_id(&a) != 1 {
        std::process::exit(1);
    }
    if type_id(&&a) != 2 {
        std::process::exit(2);
    }

    if a[0] != 10 {
        std::process::exit(3);
    }
    if a[1] != 20 {
        std::process::exit(4);
    }
    if a[2] != 30 {
        std::process::exit(5);
    }

    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        std::process::exit(6);
    }
    if std::mem::align_of_val(&a) != std::mem::align_of::<i32>() {
        std::process::exit(7);
    }

    let s = "abc";
    if type_id("abc") != 3 {
        std::process::exit(8);
    }
    let s_bytes = s.as_bytes();
    if s_bytes[0] != b'a' {
        std::process::exit(9);
    }
    if s_bytes[1] != b'b' {
        std::process::exit(10);
    }
    if s_bytes[2] != b'c' {
        std::process::exit(11);
    }
    if s_bytes[3] != b'\0' {
        std::process::exit(12);
    }

    // Successfully exit
}

fn type_id<T>(_: &T) -> u8 {
    std::any::TypeId::of::<T>()
        .into_iter()
        .fold(0u64, |sum, elem| sum.wrapping_add(elem as u64))
        as u8
}