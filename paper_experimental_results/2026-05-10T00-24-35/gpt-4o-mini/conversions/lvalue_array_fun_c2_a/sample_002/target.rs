fn id(v: i32) -> i32 {
    v + 1
}

fn type_id<T>(_: T) -> usize {
    std::any::type_name::<T>().as_ptr() as usize
}

fn main() -> i32 {
    const cx: i32 = 9;
    let ax: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(11);

    let a: [i32; 3] = [1, 2, 3];
    let pa: &[i32] = &a;

    if pa[0] != 1 {
        return 1;
    }
    if pa[1] != 2 {
        return 2;
    }
    if pa[2] != 3 {
        return 3;
    }

    if type_id(&a) != type_id(&0) {
        return 4;
    }
    if type_id(&pa[0]) != type_id(&0) {
        return 5;
    }

    if type_id(&cx) != type_id(&0) {
        return 6;
    }
    if type_id(cx) != type_id(0) {
        return 7;
    }
    if cx != 9 {
        return 8;
    }

    if type_id(ax.load(std::sync::atomic::Ordering::SeqCst)) != type_id(0) {
        return 9;
    }
    if type_id(&ax) != type_id(&0) {
        return 10;
    }
    if type_id(ax.load(std::sync::atomic::Ordering::SeqCst)) != type_id(0) {
        return 11;
    }
    if ax.load(std::sync::atomic::Ordering::SeqCst) != 11 {
        return 12;
    }

    let fp: fn(i32) -> i32 = id;
    if type_id(id(0)) != type_id(0) {
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