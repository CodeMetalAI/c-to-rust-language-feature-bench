fn main() {
    const CX: i32 = 9;
    let ax: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(11);

    let a = [1, 2, 3];

    let pa = &a;
    if pa[0]!= 1 {
        return;
    }
    if pa[1]!= 2 {
        return;
    }
    if pa[2]!= 3 {
        return;
    }

    if type_id(&a)!= 99 {
        return;
    }
    if type_id(&a[0])!= 2 {
        return;
    }

    if type_id(&CX)!= 3 {
        return;
    }
    if type_id(&CX)!= 1 {
        return;
    }
    if CX!= 9 {
        return;
    }

    if type_id(&ax)!= 4 {
        return;
    }
    if type_id(&ax)!= 5 {
        return;
    }
    if type_id(&ax.load(std::sync::atomic::Ordering::SeqCst))!= 1 {
        return;
    }
    if ax.load(std::sync::atomic::Ordering::SeqCst)!= 11 {
        return;
    }

    let fp = id;
    if type_id(&fp)!= 6 {
        return;
    }
    if fp(4)!= 5 {
        return;
    }
    if id(4)!= 5 {
        return;
    }

    if std::mem::size_of_val(&a)!= 3 * std::mem::size_of::<i32>() {
        return;
    }
    if std::mem::align_of::<i32>()!= std::mem::align_of_val(&a[0]) {
        return;
    }

    println!("Success");
}

fn id(v: i32) -> i32 {
    v + 1
}

fn type_id<T>(_x: &T) -> i32 {
    match std::any::type_name::<T>() {
        "i32" => 1,
        "*const i32" => 2,
        "*const i32" => 3,
        "std::sync::atomic::AtomicI32" => 4,
        "*const std::sync::atomic::AtomicI32" => 5,
        "fn(i32) -> i32 {id}" => 6,
        "i32" => 7,
        _ => 99,
    }
}