fn id(v: i32) -> i32 {
    v + 1
}

fn type_id<T>(_: T) -> usize {
    std::mem::size_of::<T>()
}

fn main() -> i32 {
    const cx: i32 = 9;
    let ax: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(11);

    let a: [i32; 3] = [1, 2, 3];

    let pa = &a as *const _ as *const i32;
    unsafe {
        if *pa != 1 {
            return 1;
        }
        if *pa.add(1) != 2 {
            return 2;
        }
        if *pa.add(2) != 3 {
            return 3;
        }
    }

    if type_id(&a) != std::mem::size_of::<[i32; 3]>() {
        return 4;
    }
    if type_id(&a[0]) != std::mem::size_of::<*const i32>() {
        return 5;
    }

    if type_id(&cx) != std::mem::size_of::<&i32>() {
        return 6;
    }
    if type_id(cx) != std::mem::size_of::<i32>() {
        return 7;
    }
    if cx != 9 {
        return 8;
    }

    if type_id(ax.load(std::sync::atomic::Ordering::SeqCst)) != std::mem::size_of::<i32>() {
        return 9;
    }
    if type_id(&ax) != std::mem::size_of::<&std::sync::atomic::AtomicI32>() {
        return 10;
    }
    if type_id(ax.load(std::sync::atomic::Ordering::SeqCst)) != std::mem::size_of::<i32>() {
        return 11;
    }
    if ax.load(std::sync::atomic::Ordering::SeqCst) != 11 {
        return 12;
    }

    let fp: fn(i32) -> i32 = id;
    if type_id(id(0)) != std::mem::size_of::<fn(i32) -> i32>() {
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

    return 0;
}