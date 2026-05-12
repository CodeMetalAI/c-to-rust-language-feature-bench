fn main() {
    let a: [i32; 3] = [10, 20, 30];

    if type_id(&a) != 1 {
        std::process::exit(1);
    }
    if type_id(&a as *const _ as *const [i32; 3]) != 2 {
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

    std::process::exit(0);
}

fn type_id<T>(_: T) -> usize {
    std::any::TypeId::of::<T>().hash(&mut std::collections::hash_map::DefaultHasher::new());
    99 // Just a placeholder
}