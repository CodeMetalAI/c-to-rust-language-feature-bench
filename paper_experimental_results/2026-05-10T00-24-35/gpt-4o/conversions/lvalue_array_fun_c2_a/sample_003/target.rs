fn id(v: i32) -> i32 {
    v + 1
}

fn type_id<T>(_: T) -> u32 {
    std::any::TypeId::of::<T>().hash(&mut std::collections::hash_map::DefaultHasher::new());
    match std::any::type_name::<T>() {
        "i32" => 1,
        "i32*" => 2,
        "&i32" => 2, // Equivalent pointer check in Rust
        "&i32" => 3, // For const i32 *
        "atomic::AtomicI32" => 4,
        "&atomic::AtomicI32" => 5,
        "fn(i32) -> i32" => 6,
        "const i32" => 7,
        _ => 99,
    }
}

fn main() -> i32 {
    let cx: i32 = 9;
    let ax = std::sync::atomic::AtomicI32::new(11);

    let a = [1, 2, 3];

    let pa = &a;
    if pa[0] != 1 {
        return 1;
    }
    if pa[1] != 2 {
        return 2;
    }
    if pa[2] != 3 {
        return 3;
    }

    if type_id(a) != 99 {
        return 4;
    }
    if type_id(&a[0]) != 2 {
        return 5;
    }

    if type_id(&cx) != 3 {
        return 6;
    }
    if type_id(+cx) != 1 {
        return 7;
    }
    if (+cx) != 9 {
        return 8;
    }

    if type_id(ax.load(std::sync::atomic::Ordering::Relaxed)) != 4 {
        return 9;
    }
    if type_id(&ax) != 5 {
        return 10;
    }
    if type_id(+ax.load(std::sync::atomic::Ordering::Relaxed)) != 1 {
        return 11;
    }
    if (+ax.load(std::sync::atomic::Ordering::Relaxed)) != 11 {
        return 12;
    }

    let fp: fn(i32) -> i32 = id;
    if type_id(id) != 6 {
        return 13;
    }
    if fp(4) != 5 {
        return 14;
    }
    if id(4) != 5 {
        return 15;
    }

    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        return 16;
    }
    if std::mem::align_of_val(&a[0]) != std::mem::align_of::<i32>() {
        return 17;
    }

    0
}