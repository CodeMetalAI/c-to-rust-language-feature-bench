fn main() {
    let x: i32 = 3;
    let cx: i32 = 4;
    let ax: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(5);

    if std::mem::size_of_val(&x)!= std::mem::size_of::<i32>() {
        return 1;
    }
    if std::mem::align_of_val(&x)!= std::mem::align_of::<i32>() {
        return 2;
    }

    if *(&x)!= 3 {
        return 3;
    }

    match type_id(&x) {
        1 => (),
        _ => return 4,
    }
    match type_id(&ax) {
        3 => (),
        _ => return 5,
    }

    match type_id(&cx) {
        1 => (),
        _ => return 6,
    }
    match type_id(&ax.load(std::sync::atomic::Ordering::SeqCst)) {
        1 => (),
        _ => return 7,
    }

    let y: i32 = x;
    if y!= 3 {
        return 8;
    }

    let mut x_mut: i32 = x;
    x_mut += 1;
    if x_mut!= 4 {
        return 9;
    }

    #[derive(Debug, PartialEq)]
    struct S {
        m: i32,
    }
    let mut s: S = S { m: 7 };
    if s.m!= 7 {
        return 10;
    }
    s.m = 8;
    if s.m!= 8 {
        return 11;
    }

    return 0;
}

fn type_id<T>(_x: &T) -> u8 {
    std::any::TypeId::of::<T>().as_u64().try_into().unwrap()
}