fn type_id<T: ?Sized>(x: &T) -> u32 {
    match x {
        _ if std::any::is_array(x) => 2,
        _ if std::any::is_ptr(x) => 1,
        _ if std::any::is_string(x) => 3,
        _ => 99,
    }
}

fn main() {
    let a = [10, 20, 30];
    if type_id(&a) != 2 {
        panic!("Expected type ID 2 for array, got {}", type_id(&a));
    }
    if type_id(&a as *const [i32]) != 2 {
        panic!("Expected type ID 2 for array pointer, got {}", type_id(&a as *const [i32]));
    }

    if a[0] != 10 {
        panic!("Expected first element of array to be 10, got {}", a[0]);
    }
    if a[1] != 20 {
        panic!("Expected second element of array to be 20, got {}", a[1]);
    }
    if a[2] != 30 {
        panic!("Expected third element of array to be 30, got {}", a[2]);
    }

    if a.len() != 3 {
        panic!("Expected array length to be 3, got {}", a.len());
    }
    if std::mem::size_of::<[i32; 3]>() != 3 * std::mem::size_of::<i32>() {
        panic!("Expected size of array to be 3 * size of i32, got {}",
               std::mem::size_of::<[i32; 3]>() != 3 * std::mem::size_of::<i32>());
    }

    let s = "abc";
    if type_id(s) != 3 {
        panic!("Expected type ID 3 for string, got {}", type_id(s));
    }
    if s.chars().next().unwrap() != 'a' {
        panic!("Expected first character of string to be 'a', got '{}'",
               s.chars().next().unwrap());
    }
    if s.chars().nth(1).unwrap() != 'b' {
        panic!("Expected second character of string to be 'b', got '{}'",
               s.chars().nth(1).unwrap());
    }
    if s.chars().nth(2).unwrap() != 'c' {
        panic!("Expected third character of string to be 'c', got '{}'",
               s.chars().nth(2).unwrap());
    }
    if s.chars().nth(3).unwrap() != '\0' {
        panic!("Expected fourth character of string to be '\0', got '{}'",
               s.chars().nth(3).unwrap());
    }
}