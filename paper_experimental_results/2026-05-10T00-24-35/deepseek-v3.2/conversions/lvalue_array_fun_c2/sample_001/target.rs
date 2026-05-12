use std::sync::atomic::{AtomicI32, Ordering};

fn type_id<T>(_: &T) -> u8 {
    std::any::TypeId::of::<T>()
        .hash(&mut std::collections::hash_map::DefaultHasher::new())
        .to_ne_bytes()[0]
        % 100
}

fn main() {
    let mut x = 3;
    let cx = 4;
    let ax = AtomicI32::new(5);

    if std::mem::size_of_val(&x) != std::mem::size_of::<i32>() {
        std::process::exit(1);
    }
    if std::mem::align_of_val(&x) != std::mem::align_of::<i32>() {
        std::process::exit(2);
    }

    // &x dereferenced
    if *(&x) != 3 {
        std::process::exit(3);
    }

    // Type checking simulations
    if type_id(&x) != type_id(&0_i32) {
        std::process::exit(4);
    }
    if type_id(&ax) != type_id(&AtomicI32::new(0)) {
        std::process::exit(5);
    }

    // Unary plus (does nothing in Rust but yields same type)
    let plus_cx = +cx;
    if type_id(&plus_cx) != type_id(&0_i32) {
        std::process::exit(6);
    }
    let plus_ax = ax.load(Ordering::SeqCst);
    if type_id(&plus_ax) != type_id(&0_i32) {
        std::process::exit(7);
    }

    let y = x;
    if y != 3 {
        std::process::exit(8);
    }

    x += 1;
    if x != 4 {
        std::process::exit(9);
    }

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