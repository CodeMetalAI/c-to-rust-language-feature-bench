fn id(v: i32) -> i32 {
    v + 1
}

fn main() {
    let cx = 9;
    let ax = std::sync::atomic::AtomicI32::new(11);

    let a = [1, 2, 3];

    let pa = &a;
    if pa[0]!= 1 {
        std::process::exit(1);
    }
    if pa[1]!= 2 {
        std::process::exit(2);
    }
    if pa[2]!= 3 {
        std::process::exit(3);
    }

    if std::any::type_name::<[i32; 3]>()!= "i32" {
        std::process::exit(4);
    }
    if std::any::type_name::<&i32>()!= "&i32" {
        std::process::exit(5);
    }

    if std::any::type_name::<&i32>()!= "&i32" {
        std::process::exit(6);
    }
    if cx!= 9 {
        std::process::exit(7);
    }

    if std::any::type_name::<std::sync::atomic::AtomicI32>()!= "std::sync::atomic::AtomicI32" {
        std::process::exit(8);
    }
    if std::any::type_name::<&std::sync::atomic::AtomicI32>()!= "&std::sync::atomic::AtomicI32" {
        std::process::exit(9);
    }
    if ax.load(std::sync::atomic::Ordering::SeqCst)!= 11 {
        std::process::exit(10);
    }

    let fp = id;
    if fp(4)!= 5 {
        std::process::exit(11);
    }
    if id(4)!= 5 {
        std::process::exit(12);
    }

    if std::mem::size_of_val(&a)!= 3 * std::mem::size_of::<i32>() {
        std::process::exit(13);
    }
    if std::mem::align_of::<i32>()!= std::mem::align_of_val(&a[0]) {
        std::process::exit(14);
    }
}