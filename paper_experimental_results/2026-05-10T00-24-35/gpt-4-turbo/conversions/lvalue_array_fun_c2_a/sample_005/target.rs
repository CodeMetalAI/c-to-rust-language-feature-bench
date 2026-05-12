fn main() {
    const CX: i32 = 9;
    // Rust's atomic types require explicit usage of atomic operations, but for
    // this example, we'll just simulate the atomic type for TYPE_ID equivalency.
    use std::sync::atomic::{AtomicI32, Ordering};
    let ax = AtomicI32::new(11);

    let mut a = [1, 2, 3];
    let pa = &mut a[..];
    if pa[0] != 1 {
        std::process::exit(1);
    }
    if pa[1] != 2 {
        std::process::exit(2);
    }
    if pa[2] != 3 {
        std::process::exit(3);
    }

    if type_id(&a) != "array" {
        std::process::exit(4);
    }
    if type_id(&pa[0]) != "mutable reference" {
        std::process::exit(5);
    }

    if type_id(&CX) != "immutable reference" {
        std::process::exit(6);
    }
    if type_id(CX) != "value" {
        std::process::exit(7);
    }
    if CX != 9 {
        std::process::exit(8);
    }

    if type_id(ax.load(Ordering::SeqCst)) != "value" {
        std::process::exit(9);
    }
    if type_id(&ax) != "atomic reference" {
        std::process::exit(10);
    }
    if type_id(ax.load(Ordering::SeqCst)) != "value" {
        std::process::exit(11);
    }
    if ax.load(Ordering::SeqCst) != 11 {
        std::process::exit(12);
    }

    let fp = id;
    if type_id(id as fn(i32) -> i32) != "function pointer" {
        std::process::exit(13);
    }
    if fp(4) != 5 {
        std::process::exit(14);
    }
    if id(4) != 5 {
        std::process::exit(15);
    }

    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        std::process::exit(16);
    }
    if std::mem::align_of_val(&a[0]) != std::mem::align_of::<i32>() {
        std::process::exit(17);
    }
}

fn id(v: i32) -> i32 {
    v + 1
}

fn type_id<T>(_: T) -> &'static str {
    match std::any::type_name::<T>() {
        "i32" => "value",
        "&i32" => "immutable reference",
        "&mut i32" => "mutable reference",
        "[i32; 3]" => "array",
        "fn(i32) -> i32" => "function pointer",
        "std::sync::atomic::AtomicI32" => "atomic value",
        "&std::sync::atomic::AtomicI32" => "atomic reference",
        _ => "unknown",
    }
}