use std::sync::atomic::{AtomicI32, Ordering};

fn type_id<T>(_: &T) -> u32 {
    std::any::TypeId::of::<T>().hash(&mut std::hash::DefaultHasher::new()) as u32
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

    unsafe {
        let ptr = &x as *const i32;
        if *ptr != 3 {
            std::process::exit(3);
        }
    }

    let type_id_x = type_id(&x);
    let type_id_ax = type_id(&ax);
    let type_id_cx_plus = type_id(&cx);
    let type_id_ax_plus = type_id(&ax.load(Ordering::SeqCst));

    if type_id_x != type_id(&0_i32) {
        std::process::exit(4);
    }
    if type_id_ax != type_id(&AtomicI32::new(0)) {
        std::process::exit(5);
    }
    if type_id_cx_plus != type_id(&0_i32) {
        std::process::exit(6);
    }
    if type_id_ax_plus != type_id(&0_i32) {
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