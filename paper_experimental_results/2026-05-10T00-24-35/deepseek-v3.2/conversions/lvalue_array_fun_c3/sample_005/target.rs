use std::mem;

fn type_id<T>(_x: &T) -> u32 {
    // Simulate _Generic behavior using type specialization
    std::any::TypeId::of::<T>()
        .hash(&mut std::hash::DefaultHasher::new())
        .to_string()
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap_or(99)
}

fn main() {
    let a: [i32; 3] = [10, 20, 30];

    // Check pointer type (int* vs int(*)[3]) using type_id on references
    // In C, `a` decays to `int*`, `&a` is `int(*)[3]`
    // In Rust, `&a[0]` gives `&i32`, `&a` gives `&[i32; 3]`
    if type_id(&&a[0]) != type_id(&(&[0_i32] as &[i32])) {
        return std::process::exit(1);
    }
    
    if type_id(&a) != type_id(&([0_i32; 3] as [i32; 3])) {
        return std::process::exit(2);
    }

    if a[0] != 10 {
        return std::process::exit(3);
    }
    if a[1] != 20 {
        return std::process::exit(4);
    }
    if a[2] != 30 {
        return std::process::exit(5);
    }

    if mem::size_of_val(&a) != 3 * mem::size_of::<i32>() {
        return std::process::exit(6);
    }
    if mem::align_of_val(&a) != mem::align_of::<i32>() {
        return std::process::exit(7);
    }

    let s: [char; 4] = ['a', 'b', 'c', '\0'];
    // String literal type check - in Rust string literals are &str
    if type_id(&"abc") != type_id(&("" as &str)) {
        return std::process::exit(8);
    }
    
    if s[0] != 'a' {
        return std::process::exit(9);
    }
    if s[1] != 'b' {
        return std::process::exit(10);
    }
    if s[2] != 'c' {
        return std::process::exit(11);
    }
    if s[3] != '\0' {
        return std::process::exit(12);
    }

    std::process::exit(0);
}