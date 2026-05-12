fn main() {
    let a = [10, 20, 30];

    // Check if `a` is an array of integers
    if type_id(&a) != 1 {
        std::process::exit(1);
    }
    // Check if `&a` is a reference to an array of 3 integers
    if type_id(&&a) != 2 {
        std::process::exit(2);
    }

    // Check array element values
    if a[0] != 10 {
        std::process::exit(3);
    }
    if a[1] != 20 {
        std::process::exit(4);
    }
    if a[2] != 30 {
        std::process::exit(5);
    }

    // Check size of the array
    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        std::process::exit(6);
    }
    // Check alignment of the array
    if std::mem::align_of_val(&a) != std::mem::align_of::<i32>() {
        std::process::exit(7);
    }

    let s = ['a', 'b', 'c', '\0'];
    // Check if string literal is a reference to a string slice
    if type_id("abc") != 3 {
        std::process::exit(8);
    }
    // Check string element values
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

fn type_id<T>(_: &T) -> u8 {
    std::any::TypeId::of::<T>().hash(&mut std::collections::hash_map::DefaultHasher::new()) as u8
}