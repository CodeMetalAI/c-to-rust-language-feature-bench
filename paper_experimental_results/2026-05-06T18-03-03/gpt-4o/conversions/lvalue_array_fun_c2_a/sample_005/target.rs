fn id(v: i32) -> i32 {
    v + 1
}

fn type_id<T>(_: &T) -> i32 {
    std::any::TypeId::of::<T>().hash() as i32
}

fn main() {
    let cx: i32 = 9;
    let ax: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(11);

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

    if type_id(&a) != type_id(&[0; 3]) {
        std::process::exit(4);
    }
    if type_id(&a[0]) != type_id(&0) {
        std::process::exit(5);
    }

    if type_id(&cx) != type_id(&0) {
        std::process::exit(6);
    }
    if type_id(&(cx + 0)) != type_id(&0) {
        std::process::exit(7);
    }
    if (cx + 0) != 9 {
        std::process::exit(8);
    }

    if type_id(&ax) != type_id(&std::sync::atomic::AtomicI32::new(0)) {
        std::process::exit(9);
    }
    if type_id(&ax) != type_id(&std::sync::atomic::AtomicI32::new(0)) {
        std::process::exit(10);
    }
    if type_id(&(ax.load(std::sync::atomic::Ordering::SeqCst) + 0)) != type_id(&0) {
        std::process::exit(11);
    }
    if (ax.load(std::sync::atomic::Ordering::SeqCst) + 0) != 11 {
        std::process::exit(12);
    }

    let fp: fn(i32) -> i32 = id;
    if type_id(&id) != type_id(&fp) {
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

    std::process::exit(0);
}