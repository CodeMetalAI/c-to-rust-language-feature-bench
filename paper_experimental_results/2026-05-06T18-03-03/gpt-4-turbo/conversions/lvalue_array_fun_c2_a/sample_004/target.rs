fn main() {
    let cx = 9;
    let ax = std::sync::atomic::AtomicI32::new(11);

    let mut a = [1, 2, 3];

    let pa = &mut a;
    if pa[0] != 1 {
        std::process::exit(1);
    }
    if pa[1] != 2 {
        std::process::exit(2);
    }
    if pa[2] != 3 {
        std::process::exit(3);
    }

    // Rust does not have a direct equivalent to C's _Generic, so we use functions to simulate it
    if type_id_array(&a) != 99 {
        std::process::exit(4);
    }
    if type_id(&a[0]) != 2 {
        std::process::exit(5);
    }

    if type_id(&cx) != 3 {
        std::process::exit(6);
    }
    if type_id(cx) != 1 {
        std::process::exit(7);
    }
    if cx != 9 {
        std::process::exit(8);
    }

    if type_id_atomic_i32(&ax) != 4 {
        std::process::exit(9);
    }
    if type_id(&ax) != 5 {
        std::process::exit(10);
    }
    if type_id(ax.load(std::sync::atomic::Ordering::SeqCst)) != 1 {
        std::process::exit(11);
    }
    if ax.load(std::sync::atomic::Ordering::SeqCst) != 11 {
        std::process::exit(12);
    }

    let fp = id;
    if type_id_fn(id) != 6 {
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

fn type_id<T>(_x: &T) -> i32 {
    match std::any::type_name::<T>() {
        "i32" => 1,
        "&i32" => 2,
        "&i32" => 3, // Rust does not differentiate between const and non-const pointers
        "std::sync::atomic::AtomicI32" => 4,
        "&std::sync::atomic::AtomicI32" => 5,
        _ => 99,
    }
}

fn type_id_array<T>(_x: &[T]) -> i32 {
    99 // Arrays decay to pointers in C, but not in Rust
}

fn type_id_fn<T>(_x: T) -> i32 where T: Fn(i32) -> i32 {
    6
}

fn type_id_atomic_i32(_x: &std::sync::atomic::AtomicI32) -> i32 {
    4
}