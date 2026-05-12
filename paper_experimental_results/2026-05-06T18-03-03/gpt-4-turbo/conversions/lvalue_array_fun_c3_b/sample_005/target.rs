fn main() {
    let mut a: [i32; 3] = [10, 20, 30];

    // In Rust, we don't have a direct equivalent of C's _Generic, but we can use types and traits
    // to achieve similar type-based behavior. Here we use a simple function to mimic TYPE_ID.
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
    if s.as_bytes()[0] != b'a' {
        std::process::exit(9);
    }
    if s.as_bytes()[1] != b'b' {
        std::process::exit(10);
    }
    if s.as_bytes()[2] != b'c' {
        std::process::exit(11);
    }
    if s.as_bytes()[3] != b'\0' {
        std::process::exit(12);
    }

    // Exit code 0 for success
}

fn type_id<T>(_: &T) -> i32 {
    use std::any::TypeId;
    if TypeId::of::<&[i32; 3]>() == TypeId::of::<T>() {
        2
    } else if TypeId::of::<&[i32]>() == TypeId::of::<T>() {
        1
    } else if TypeId::of::<&str>() == TypeId::of::<T>() {
        3
    } else {
        99
    }
}