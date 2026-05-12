fn type_id<T>(_: T) -> u32 {
    match std::any::type_name::<T>() {
        "std::primitive::int" => 1,
        "[i32; 3]" => 2,
        "&'static str" => 3,
        "[char; 4]" => 4,
        _ => 99,
    }
}

fn main() -> i32 {
    let a: [i32; 3] = [10, 20, 30];

    if type_id(&a) != 1 {
        return 1;
    }
    if type_id(&a) != 2 {
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

    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        return 6;
    }
    if std::mem::align_of_val(&a) != std::mem::align_of::<i32>() {
        return 7;
    }

    let s: [char; 4] = ['a', 'b', 'c', '\0'];
    if type_id("abc") != 3 {
        return 8;
    }
    if s[0] != 'a' {
        return 9;
    }
    if s[1] != 'b' {
        return 10;
    }
    if s[2] != 'c' {
        return 11;
    }
    if s[3] != '\0' {
        return 12;
    }

    0
}