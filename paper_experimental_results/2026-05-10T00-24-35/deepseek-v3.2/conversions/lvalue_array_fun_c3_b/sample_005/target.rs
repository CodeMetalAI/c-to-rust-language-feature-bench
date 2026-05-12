fn type_id<T>(_: &T) -> u8 {
    use std::any::TypeId;
    
    if TypeId::of::<*const i32>() == TypeId::of::<T>() {
        1
    } else if TypeId::of::<*const [i32; 3]>() == TypeId::of::<T>() {
        2
    } else if TypeId::of::<&str>() == TypeId::of::<T>() {
        3
    } else if TypeId::of::<[char; 4]>() == TypeId::of::<T>() {
        4
    } else {
        99
    }
}

fn main() {
    let a = [10, 20, 30];
    
    // Check type of array reference (decays to pointer in C)
    if type_id(&(&a[0] as *const i32)) != 1 {
        std::process::exit(1);
    }
    
    // Check type of pointer to array
    if type_id(&(&a as *const [i32; 3])) != 2 {
        std::process::exit(2);
    }
    
    // Check array values
    if a[0] != 10 {
        std::process::exit(3);
    }
    if a[1] != 20 {
        std::process::exit(4);
    }
    if a[2] != 30 {
        std::process::exit(5);
    }
    
    // Check size
    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        std::process::exit(6);
    }
    
    // Check alignment
    if std::mem::align_of_val(&a) != std::mem::align_of::<i32>() {
        std::process::exit(7);
    }
    
    // Check string literal type
    if type_id(&"abc") != 3 {
        std::process::exit(8);
    }
    
    // Check array string
    let s = ['a' as u8, 'b' as u8, 'c' as u8, 0];
    if s[0] != b'a' {
        std::process::exit(9);
    }
    if s[1] != b'b' {
        std::process::exit(10);
    }
    if s[2] != b'c' {
        std::process::exit(11);
    }
    if s[3] != 0 {
        std::process::exit(12);
    }
    
    std::process::exit(0);
}