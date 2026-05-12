fn type_id<T>(_: T) -> usize {
    std::any::TypeId::of::<T>().hash(&mut std::collections::hash_map::DefaultHasher::new());
    99 // Default case
}

fn type_id_int(_: i32) -> usize {
    1
}

fn type_id_int_ptr(_: *const i32) -> usize {
    3
}

fn type_id_int_ref(_: &i32) -> usize {
    3
}

fn type_id_atomic_int(_: std::sync::atomic::AtomicI32) -> usize {
    4
}

fn type_id_atomic_int_ptr(_: *const std::sync::atomic::AtomicI32) -> usize {
    5
}

fn type_id_fn(_: fn(i32) -> i32) -> usize {
    6
}

fn type_id_const_int(_: &i32) -> usize {
    7
}

fn main() -> i32 {
    const CX: i32 = 9;
    let ax = std::sync::atomic::AtomicI32::new(11);

    let a: [i32; 3] = [1, 2, 3];

    let pa = &a as *const _ as *const i32;
    if unsafe { *pa } != 1 {
        return 1;
    }
    if unsafe { *pa.add(1) } != 2 {
        return 2;
    }
    if unsafe { *pa.add(2) } != 3 {
        return 3;
    }

    if type_id(&a) != 99 {
        return 4;
    }
    if type_id_int_ptr(&a[0] as *const _) != 2 {
        return 5;
    }

    if type_id_const_int(&CX) != 3 {
        return 6;
    }
    if type_id_int(CX) != 1 {
        return 7;
    }
    if CX != 9 {
        return 8;
    }

    if type_id_atomic_int(ax) != 4 {
        return 9;
    }
    if type_id_atomic_int_ptr(&ax) != 5 {
        return 10;
    }
    if type_id_int(ax.load(std::sync::atomic::Ordering::SeqCst)) != 1 {
        return 11;
    }
    if ax.load(std::sync::atomic::Ordering::SeqCst) != 11 {
        return 12;
    }

    let fp: fn(i32) -> i32 = id;
    if type_id_fn(id) != 6 {
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
    if std::mem::align_of::<i32>() != std::mem::align_of_val(&a[0]) {
        return 17;
    }

    0
}

fn id(v: i32) -> i32 {
    v + 1
}