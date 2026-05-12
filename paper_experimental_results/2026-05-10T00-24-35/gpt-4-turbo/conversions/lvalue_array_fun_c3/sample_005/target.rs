fn type_id<T>(_: &T) -> usize {
    if std::any::TypeId::of::<T>() == std::any::TypeId::of::<&[i32; 3]>() {
        2
    } else if std::any::TypeId::of::<T>() == std::any::TypeId::of::<&str>() {
        3
    } else if std::any::TypeId::of::<T>() == std::any::TypeId::of::<&[char; 4]>() {
        4
    } else {
        99
    }
}

fn main() {
    let a: [i32; 3] = [10, 20, 30];
    let a_ref = &a;

    if type_id(&a) != 99 {
        std::process::exit(1);
    }
    if type_id(&a_ref) != 2 {
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

    let s: [char; 4] = ['a', 'b', 'c', '\0'];
    if type_id("abc") != 3 {
        std::process::exit(8);
    }
    if s[0] != 'a' {
        std::process::exit(9);
    }
    if s[1] != 'b' {
        std::process::exit(10);
    }
    if s[2] != 'c' {
        std::process::exit(11);
    }
    if s[3] != '\0' {
        std::process::exit(12);
    }
}