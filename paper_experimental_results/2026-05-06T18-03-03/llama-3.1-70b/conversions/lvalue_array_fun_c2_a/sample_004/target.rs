fn main() {
    const CX: i32 = 9;
    let ax = std::sync::atomic::AtomicI32::new(11);

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

    if std::any::type_name::<[i32; 3]>()!= "[i32; 3]" {
        return;
    }
    if std::any::type_name::<&i32>()!= "&i32" {
        return;
    }

    if std::any::type_name::<&i32>()!= "&i32" {
        return;
    }
    if CX!= 9 {
        return;
    }

    if std::any::type_name::<std::sync::atomic::AtomicI32>()!= "std::sync::atomic::AtomicI32" {
        return;
    }
    if std::any::type_name::<&std::sync::atomic::AtomicI32>()!= "&std::sync::atomic::AtomicI32" {
        return;
    }
    if ax.load(std::sync::atomic::Ordering::SeqCst)!= 11 {
        return;
    }

    let fp = id;
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
}

fn id(v: i32) -> i32 {
    v + 1
}