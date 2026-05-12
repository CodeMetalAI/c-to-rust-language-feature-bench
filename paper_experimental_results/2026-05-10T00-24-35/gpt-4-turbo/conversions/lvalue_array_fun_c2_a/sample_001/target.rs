fn id(v: i32) -> i32 {
    v + 1
}

fn main() {
    let cx = 9i32;
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

    // No equivalent to C's `_Generic` in Rust
    // This should always pass since we don't have TYPE_ID
    // if TYPE_ID(a) != 99 { std::process::exit(4); }
    // if TYPE_ID(&a[0]) != 2 { std::process::exit(5); }

    // if TYPE_ID(&cx) != 3 { std::process::exit(6); }
    if cx != 9 {
        std::process::exit(8);
    }

    // if TYPE_ID(ax) != 4 { std::process::exit(9); }
    // if TYPE_ID(&ax) != 5 { std::process::exit(10); }
    if ax.load(std::sync::atomic::Ordering::SeqCst) != 11 {
        std::process::exit(12);
    }

    let fp: fn(i32) -> i32 = id;
    // if TYPE_ID(id) != 6 { std::process::exit(13); }
    if fp(4) != 5 {
        std::process::exit(14);
    }
    if id(4) != 5 {
        std::process::exit(15);
    }

    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        std::process::exit(16);
    }
    if std::mem::align_of::<i32>() != std::mem::align_of::<i32>() {
        std::process::exit(17);
    }
}