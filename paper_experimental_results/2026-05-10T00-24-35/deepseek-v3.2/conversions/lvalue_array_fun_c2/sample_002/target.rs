use std::sync::atomic::{AtomicI32, Ordering};

fn type_id<T>(_x: &T) -> i32 {
    std::any::TypeId::of::<T>()
        .hash(&mut std::collections::hash_map::DefaultHasher::new())
        .to_string()
        .parse::<i32>()
        .unwrap_or(99) % 100
}

fn main() {
    let mut x: i32 = 3;
    let cx: i32 = 4;
    let ax = AtomicI32::new(5);

    // Check size and alignment
    if std::mem::size_of_val(&x) != std::mem::size_of::<i32>() {
        std::process::exit(1);
    }
    if std::mem::align_of_val(&x) != std::mem::align_of::<i32>() {
        std::process::exit(2);
    }

    // Check dereferencing
    unsafe {
        let ptr = &x as *const i32;
        if *ptr != 3 {
            std::process::exit(3);
        }
    }

    // Check type IDs
    if type_id(&x) != type_id(&0i32) {
        std::process::exit(4);
    }
    if type_id(&ax) != type_id(&AtomicI32::new(0)) {
        std::process::exit(5);
    }

    // Check unary plus conversions
    let plus_cx = cx;
    if type_id(&plus_cx) != type_id(&x) {
        std::process::exit(6);
    }
    let plus_ax = ax.load(Ordering::SeqCst);
    if type_id(&plus_ax) != type_id(&x) {
        std::process::exit(7);
    }

    // Check assignment
    let y = x;
    if y != 3 {
        std::process::exit(8);
    }

    // Check increment
    x += 1;
    if x != 4 {
        std::process::exit(9);
    }

    // Check struct
    struct S {
        m: i32,
    }
    let mut s = S { m: 7 };
    if s.m != 7 {
        std::process::exit(10);
    }
    s.m = 8;
    if s.m != 8 {
        std::process::exit(11);
    }

    std::process::exit(0);
}