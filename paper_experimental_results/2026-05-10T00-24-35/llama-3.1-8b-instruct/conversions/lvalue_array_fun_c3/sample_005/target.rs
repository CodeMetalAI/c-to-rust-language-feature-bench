fn type_id<T>(x: &T) -> u32
where
    T:?Sized,
{
    match std::any::type_name(&x) {
        "int *" => 1,
        "int (*)[3]" => 2,
        "const char *" => 3,
        "const [u8; 4]" => 4,
        _ => 99,
    }
}

fn main() {
    let a: [i32; 3] = [10, 20, 30];

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
    if std::mem::align_of::<[i32; 3]>()!= std::mem::align_of::<i32>() {
        return;
    }

    let s = "abc";
    if type_id(&s)!= 3 {
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

    println!("All tests passed");
}