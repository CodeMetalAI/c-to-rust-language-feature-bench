fn type_id<T: Copy>(x: &T) -> i32 {
    match x {
        _ if std::mem::size_of::<T>() == std::mem::size_of::<i32>() => 1,
        _ if std::mem::size_of::<T>() == std::mem::size_of::<i32>() * 3 => 2,
        _ if std::mem::size_of::<T>() == std::mem::size_of::<&str>() => 3,
        _ if std::mem::size_of::<T>() == std::mem::size_of::<&str>() * 4 => 4,
        _ => 99,
    }
}

fn main() {
    let a = [10, 20, 30];

    if type_id(&a)!= 1 {
        return;
    }
    if type_id(&a as *const [i32; 3])!= 2 {
        return;
    }

    if a[0]!= 10 {
        return;
    }
    if a[1]!= 20 {
        return;
    }
    if a[2]!= 30 {
        return;
    }

    if a.len()!= 3 {
        return;
    }
    if std::mem::align_of::<i32>()!= std::mem::align_of::<usize>() {
        return;
    }

    let s = "abc";
    if type_id(s.as_ptr())!= 3 {
        return;
    }
    if s.chars().nth(0).unwrap()!= 'a' {
        return;
    }
    if s.chars().nth(1).unwrap()!= 'b' {
        return;
    }
    if s.chars().nth(2).unwrap()!= 'c' {
        return;
    }
    if s.chars().nth(3).unwrap()!= '\0' {
        return;
    }

    assert!(true);
}