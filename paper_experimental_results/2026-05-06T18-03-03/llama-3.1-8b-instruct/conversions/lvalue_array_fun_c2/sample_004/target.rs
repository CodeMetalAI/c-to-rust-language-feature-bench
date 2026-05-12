fn main() {
    let x: i32 = 3;
    let cx: i32 = 4;
    let ax: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(5);

    if std::mem::size_of::<i32>() != std::mem::size_of_val(&x) {
        return 1;
    }
    if std::mem::align_of::<i32>() != std::mem::align_of_val(&x) {
        return 2;
    }

    if *(&x) != 3 {
        return 3;
    }

    let type_id = match x {
        _ => 99,
    };
    if type_id != 1 {
        return 4;
    }
    let type_id = match ax.load(std::sync::atomic::Ordering::SeqCst) {
        _ => 99,
    };
    if type_id != 3 {
        return 5;
    }

    let type_id = match +cx {
        _ => 99,
    };
    if type_id != 1 {
        return 6;
    }
    let type_id = match ax.load(std::sync::atomic::Ordering::SeqCst) {
        _ => 99,
    };
    if type_id != 1 {
        return 7;
    }

    let y = x;
    if y != 3 {
        return 8;
    }

    x += 1;
    if x != 4 {
        return 9;
    }

    struct S {
        m: i32,
    }
    let s = S { m: 7 };
    if s.m != 7 {
        return 10;
    }
    s.m = 8;
    if s.m != 8 {
        return 11;
    }

    return 0;
}