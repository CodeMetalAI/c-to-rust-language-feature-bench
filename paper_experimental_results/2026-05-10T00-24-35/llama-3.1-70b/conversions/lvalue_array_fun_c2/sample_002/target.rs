fn type_id<T>() -> u32 {
    match std::mem::discriminant(&std::any::TypeId::of::<T>()) {
        std::any::TypeId::of::<i32>() => 1,
        std::any::TypeId::of::<u32>() => 2,
        std::any::TypeId::of::<std::sync::atomic::AtomicI32>() => 3,
        _ => 99,
    }
}

struct S {
    m: i32,
}

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

    if type_id::<i32>() != 1 {
        std::process::exit(4);
    }
    if type_id::<std::sync::atomic::AtomicI32>() != 3 {
        std::process::exit(5);
    }

    if type_id::<i32>() != 1 {
        std::process::exit(6);
    }
    if type_id::<i32>() != 1 {
        std::process::exit(7);
    }

    let y = x;
    if y != 3 {
        std::process::exit(8);
    }

    let mut x_mut = x;
    x_mut += 1;
    if x_mut != 4 {
        std::process::exit(9);
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