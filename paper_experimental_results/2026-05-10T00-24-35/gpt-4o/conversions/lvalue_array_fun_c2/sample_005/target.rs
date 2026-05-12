fn main() {
    let x: i32 = 3;
    let cx: i32 = 4;
    let ax: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(5);

    if std::mem::size_of_val(&x) != std::mem::size_of::<i32>() {
        std::process::exit(1);
    }
    if std::mem::align_of_val(&x) != std::mem::align_of::<i32>() {
        std::process::exit(2);
    }

    if *(&x) != 3 {
        std::process::exit(3);
    }

    if type_id(&x) != 1 {
        std::process::exit(4);
    }
    if type_id(&ax) != 3 {
        std::process::exit(5);
    }

    // The unary plus does nothing in Rust, thus we just use cx directly
    if type_id(&cx) != 1 {
        std::process::exit(6);
    }
    // Retrieve the inner value of ax for type checking
    if type_id(&ax.load(std::sync::atomic::Ordering::Relaxed)) != 1 {
        std::process::exit(7);
    }

    let mut y = x;
    if y != 3 {
        std::process::exit(8);
    }

    let mut x = x;
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

fn type_id<T: 'static>(_: &T) -> i32 {
    if std::any::TypeId::of::<T>() == std::any::TypeId::of::<i32>() {
        1
    } else if std::any::TypeId::of::<T>() == std::any::TypeId::of::<u32>() {
        2
    } else if std::any::TypeId::of::<T>() == std::any::TypeId::of::<std::sync::atomic::AtomicI32>() {
        3
    } else {
        99
    }
}