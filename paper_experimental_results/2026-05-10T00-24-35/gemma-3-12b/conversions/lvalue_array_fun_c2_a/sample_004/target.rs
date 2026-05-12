#[inline]
fn type_id<T>(_: &T) -> i32 {
    _generic_type_id::<T>()
}

#[inline]
fn _generic_type_id<T>() -> i32 {
    let type_ = std::any::TypeId::new::<T>();
    match type_ {
        TypeId::of::<i32>() => 1,
        TypeId::of::<*mut i32>() => 2,
        TypeId::of::<*const i32>() => 3,
        TypeId::of::<std::sync::atomic::AtomicI32>() => 4,
        TypeId::of::<*mut std::sync::atomic::AtomicI32>() => 5,
        TypeId::of::<fn(i32) -> i32>() => 6,
        TypeId::of::<i32>() if std::ptr::addr_of_mut(&0i32) == std::ptr::addr_of_mut(&std::sync::atomic::AtomicI32::new(0)) => 7,
        _ => 99,
    }
}

#[inline]
fn id(v: i32) -> i32 {
    v + 1
}

fn main() {
    const cx: i32 = 9;
    let ax: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(11);

    let a: [i32; 3] = [1, 2, 3];

    let pa = a.as_ptr();
    if pa.add(0).read_unaligned() != 1 {
        std::process::exit(1);
    }
    if pa.add(1).read_unaligned() != 2 {
        std::process::exit(2);
    }
    if pa.add(2).read_unaligned() != 3 {
        std::process::exit(3);
    }

    if type_id(&a) != 99 {
        std::process::exit(4);
    }
    if type_id(&a[0]) != 2 {
        std::process::exit(5);
    }

    if type_id(&cx) != 3 {
        std::process::exit(6);
    }
    if type_id(&cx + 0) != 1 {
        std::process::exit(7);
    }
    if cx + 0 != 9 {
        std::process::exit(8);
    }

    if type_id(&ax) != 4 {
        std::process::exit(9);
    }
    if type_id(&ax) != 5 {
        std::process::exit(10);
    }
    if type_id(&ax.load(std::sync::atomic::Ordering::Relaxed)) != 1 {
        std::process::exit(11);
    }
    if ax.load(std::sync::atomic::Ordering::Relaxed) != 11 {
        std::process::exit(12);
    }

    let fp: fn(i32) -> i32 = id;
    if type_id(&id) != 6 {
        std::process::exit(13);
    }
    if fp(4) != 5 {
        std::process::exit(14);
    }
    if id(4) != 5 {
        std::process::exit(15);
    }

    if std::mem::size_of::<[i32; 3]>() != 3 * std::mem::size_of::<i32>() {
        std::process::exit(16);
    }
    if std::mem::align_of::<i32>() != std::mem::align_of::<i32>() {
        std::process::exit(17);
    }

    std::process::exit(0);
}