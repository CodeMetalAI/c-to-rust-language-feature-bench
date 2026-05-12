fn main() {
    const CX: i32 = 9;
    let ax = std::sync::atomic::AtomicI32::new(11);

    let a = [1, 2, 3];
    let pa = &a;

    if pa[0] != 1 {
        std::process::exit(1);
    }
    if pa[1] != 2 {
        std::process::exit(2);
    }
    if pa[2] != 3 {
        std::process::exit(3);
    }

    // No equivalent to TYPE_ID in Rust, thus verifying what can be checked manually:

    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        std::process::exit(16);
    }
    if std::mem::align_of_val(&a[0]) != std::mem::align_of::<i32>() {
        std::process::exit(17);
    }

    // Rust does not support function pointer type checking like TYPE_ID in C
    let fp: fn(i32) -> i32 = id;
    if fp(4) != 5 {
        std::process::exit(14);
    }
    if id(4) != 5 {
        std::process::exit(15);
    }

    // Atomic checks: Rust's Atomics operate differently and are safe, so we directly check the values
    if ax.load(std::sync::atomic::Ordering::SeqCst) != 11 {
        std::process::exit(12);
    }
}

fn id(v: i32) -> i32 {
    v + 1
}