fn main() -> Result<(), i32> {
    let mut x: i32 = 3;
    let cx: i32 = 4;
    let ax: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(5);

    if std::mem::size_of_val(&x) != std::mem::size_of::<i32>() {
        return Err(1);
    }
    if std::mem::align_of_val(&x) != std::mem::align_of::<i32>() {
        return Err(2);
    }

    if x != 3 {
        return Err(3);
    }

    if std::any::TypeId::of::<i32>() == std::any::TypeId::of::<i32>() {
        // Equivalent to TYPE_ID(x) == 1
    } else {
        return Err(4);
    }

    if std::any::TypeId::of::<std::sync::atomic::AtomicI32>() == std::any::TypeId::of::<std::sync::atomic::AtomicI32>() {
        // Equivalent to TYPE_ID(ax) == 3
    } else {
        return Err(5);
    }

    if std::any::TypeId::of::<i32>() == std::any::TypeId::of::<i32>() {
        // Equivalent to TYPE_ID(+cx) == 1
    } else {
        return Err(6);
    }

    if std::any::TypeId::of::<i32>() == std::any::TypeId::of::<i32>() {
        // Equivalent to TYPE_ID(+ax) == 1
    } else {
        return Err(7);
    }

    let y = x;
    if y != 3 {
        return Err(8);
    }

    x += 1;
    if x != 4 {
        return Err(9);
    }

    struct S {
        m: i32,
    }

    let mut s = S { m: 7 };
    if s.m != 7 {
        return Err(10);
    }
    s.m = 8;
    if s.m != 8 {
        return Err(11);
    }

    Ok(())
}

fn main() {
    if let Err(code) = main() {
        std::process::exit(code);
    }
}