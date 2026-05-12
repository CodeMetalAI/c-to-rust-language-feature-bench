fn type_id<T>(_: T) -> u32 {
    match std::any::type_id_of::<T>() {
        std::any::TypeId::of::<&'static str>() => 3,
        std::any::TypeId::of::<&str>() => 3,
        std::any::TypeId::of::<String>() => 3,
        std::any::TypeId::of::<[u8; 4]>() => 4,
        std::any::TypeId::of::<&[u8; 4]>() => 4,
        std::any::TypeId::of::<[u8; 4]>() => 4,
        std::any::TypeId::of::<&[u8]>() => 4,
        std::any::TypeId::of::<&[u8]>() => 4,
        std::any::TypeId::of::<[u8]>() => 4,
        std::any::TypeId::of::<[i32; 3]>() => 2,
        std::any::TypeId::of::<&[i32; 3]>() => 2,
        std::any::TypeId::of::<&mut [i32; 3]>() => 2,
        std::any::TypeId::of::<i32>() => 1,
        std::any::TypeId::of::<&i32>() => 1,
        std::any::TypeId::of::<&mut i32>() => 1,
        _ => 99,
    }
}

fn main() {
    let a: [i32; 3] = [10, 20, 30];

    if type_id(a) != 1 {
        println!("Error 1");
        return;
    }
    if type_id(&a) != 2 {
        println!("Error 2");
        return;
    }

    if a[0] != 10 {
        println!("Error 3");
        return;
    }
    if a[1] != 20 {
        println!("Error 4");
        return;
    }
    if a[2] != 30 {
        println!("Error 5");
        return;
    }

    if a.len() != 3 {
        println!("Error 6");
        return;
    }
    if std::mem::size_of::<[i32; 3]>() != 3 * std::mem::size_of::<i32>() {
        println!("Error 6");
        return;
    }

    let s = [b'a', b'b', b'c', 0];

    if type_id(&s) != 4 {
        println!("Error 8");
        return;
    }
    if s[0] != b'a' {
        println!("Error 9");
        return;
    }
    if s[1] != b'b' {
        println!("Error 10");
        return;
    }
    if s[2] != b'c' {
        println!("Error 11");
        return;
    }
    if s[3] != 0 {
        println!("Error 12");
        return;
    }

    println!("Success");
}