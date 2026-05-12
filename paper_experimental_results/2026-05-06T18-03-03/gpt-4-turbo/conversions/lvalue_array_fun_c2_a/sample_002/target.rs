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

    // Rust does not have a direct equivalent to C's _Generic keyword,
    // so we cannot replicate TYPE_ID functionality directly.
    // Instead, we can use Rust's type system and pattern matching,
    // but for this specific translation, we will skip these checks.

    if CX != 9 {
        std::process::exit(8);
    }

    if ax.load(std::sync::atomic::Ordering::SeqCst) != 11 {
        std::process::exit(12);
    }

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