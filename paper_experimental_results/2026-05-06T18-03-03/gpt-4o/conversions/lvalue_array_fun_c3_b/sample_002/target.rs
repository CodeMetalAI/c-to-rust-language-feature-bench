fn main() {
    let a = [10, 20, 30];

    // In Rust, arrays are not automatically converted to pointers, so we need to check types differently.
    // We use type_id to simulate the TYPE_ID macro behavior.
    fn type_id<T>(_: &T) -> u8 {
        std::any::TypeId::of::<T>().hash(&mut std::collections::hash_map::DefaultHasher::new()) as u8
    }

    if type_id(&a) != type_id(&[0; 3]) {
        std::process::exit(1);
    }
    if type_id(&&a) != type_id(&&[0; 3]) {
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

    let s = ['a', 'b', 'c', '\0'];
    if type_id(&"abc") != type_id(&"abc") {
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