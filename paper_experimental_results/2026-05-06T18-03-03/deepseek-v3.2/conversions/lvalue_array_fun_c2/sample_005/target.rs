use std::sync::atomic::{AtomicI32, Ordering};

fn type_id<T>(_: &T) -> u32 {
    std::any::TypeId::of::<T>().hash(&mut std::hash::BuildHasherDefault::<std::collections::hash_map::DefaultHasher>::default().build_hasher()) as u32
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

    if *(&x) != 3 {
        std::process::exit(3);
    }

    // Simulate TYPE_ID checks using type IDs
    let type_id_x = type_id(&x);
    let type_id_i32 = type_id(&0_i32);
    if type_id_x != type_id_i32 {
        std::process::exit(4);
    }

    let type_id_ax = type_id(&ax);
    let type_id_atomic_i32 = type_id(&AtomicI32::new(0));
    if type_id_ax != type_id_atomic_i32 {
        std::process::exit(5);
    }

    // +cx and +ax produce i32 in Rust (integer promotion)
    let promoted_cx = cx;
    let type_id_promoted_cx = type_id(&promoted_cx);
    if type_id_promoted_cx != type_id_i32 {
        std::process::exit(6);
    }

    let promoted_ax = ax.load(Ordering::SeqCst);
    let type_id_promoted_ax = type_id(&promoted_ax);
    if type_id_promoted_ax != type_id_i32 {
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