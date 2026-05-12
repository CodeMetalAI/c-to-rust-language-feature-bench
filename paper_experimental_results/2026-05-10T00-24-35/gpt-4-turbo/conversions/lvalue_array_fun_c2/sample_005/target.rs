fn main() {
    let mut x = 3;
    let cx = 4;
    let ax = std::sync::atomic::AtomicI32::new(5);

    if std::mem::size_of::<i32>() != std::mem::size_of::<i32>() {
        std::process::exit(1);
    }
    if std::mem::align_of::<i32>() != std::mem::align_of::<i32>() {
        std::process::exit(2);
    }

    if *&x != 3 {
        std::process::exit(3);
    }

    // Simulating _Generic with Rust match expressions
    let type_id_x = match () {
        _ if std::any::TypeId::of::<i32>() == std::any::TypeId::of::<i32>() => 1,
        _ => 99,
    };
    let type_id_ax = match () {
        _ if std::any::TypeId::of::<i32>() == std::any::TypeId::of::<std::sync::atomic::AtomicI32>() => 3,
        _ => 99,
    };

    if type_id_x != 1 {
        std::process::exit(4);
    }
    if type_id_ax != 3 {
        std::process::exit(5);
    }

    let type_id_plus_cx = match () {
        _ if std::any::TypeId::of::<i32>() == std::any::TypeId::of::<i32>() => 1,
        _ => 99,
    };
    let type_id_plus_ax = match () {
        _ if std::any::TypeId::of::<i32>() == std::any::TypeId::of::<i32>() => 1,
        _ => 99,
    };

    if type_id_plus_cx != 1 {
        std::process::exit(6);
    }
    if type_id_plus_ax != 1 {
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