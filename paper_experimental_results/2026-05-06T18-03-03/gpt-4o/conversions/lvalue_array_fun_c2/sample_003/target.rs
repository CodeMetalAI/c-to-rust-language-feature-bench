fn main() {
    let mut x: i32 = 3;
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

    fn type_id<T>(_: &T) -> u32 {
        std::any::TypeId::of::<T>().hash(&mut std::collections::hash_map::DefaultHasher::new()) as u32
    }

    if type_id(&x) != type_id(&3i32) {
        std::process::exit(4);
    }
    if type_id(&ax) != type_id(&std::sync::atomic::AtomicI32::new(0)) {
        std::process::exit(5);
    }

    if type_id(&(cx + 0)) != type_id(&3i32) {
        std::process::exit(6);
    }
    if type_id(&(ax.load(std::sync::atomic::Ordering::SeqCst) + 0)) != type_id(&3i32) {
        std::process::exit(7);
    }

    let mut y = x;
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
}