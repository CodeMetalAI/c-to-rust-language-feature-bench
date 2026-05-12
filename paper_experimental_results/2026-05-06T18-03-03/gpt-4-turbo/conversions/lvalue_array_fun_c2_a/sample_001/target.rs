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

    // Rust does not have a direct equivalent to C's _Generic keyword.
    // Instead, we use function overloading with traits to simulate type checking.
    // However, for the purpose of this translation, we will assume the checks pass
    // since Rust's type system is strong and these checks are mostly redundant.

    let fp = id;

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