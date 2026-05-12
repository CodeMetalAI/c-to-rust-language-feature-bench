fn main() {
    let x: i32 = 3;
    let cx: i32 = 4;
    let ax: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(5);

    if std::mem::size_of_val(&x) != std::mem::size_of::<i32>() {
        return;
    }
    if std::mem::align_of_val(&x) != std::mem::align_of::<i32>() {
        return;
    }

    if *(&x) != 3 {
        return;
    }

    if type_id(&x) != 1 {
        return;
    }
    if type_id(&ax) != 3 {
        return;
    }

    if type_id(&cx) != 1 {
        return;
    }
    if type_id(&ax.load(std::sync::atomic::Ordering::SeqCst)) != 1 {
        return;
    }

    let y = x;
    if y != 3 {
        return;
    }

    let mut x = x;
    x += 1;
    if x != 4 {
        return;
    }

    #[derive(Debug, PartialEq)]
    struct S {
        m: i32,
    }
    let mut s = S { m: 7 };
    if s.m != 7 {
        return;
    }
    s.m = 8;
    if s.m != 8 {
        return;
    }
}

fn type_id<T: ?Sized>(_x: &T) -> u8 {
    std::any::TypeId::of::<T>().hash_code() % 100
}