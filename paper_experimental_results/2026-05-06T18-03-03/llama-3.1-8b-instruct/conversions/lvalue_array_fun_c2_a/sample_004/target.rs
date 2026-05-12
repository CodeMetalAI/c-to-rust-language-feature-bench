#![allow(unused)]

fn type_id<T>(_: T) -> u32 {
    match std::any::type_id_of::<T>() {
        std::any::TypeId::Int => 1,
        std::any::TypeId::Ptr(_) => 2,
        std::any::TypeId::ConstPtr(_) => 3,
        std::any::TypeId::Atomic(_) => 4,
        std::any::TypeId::AtomicPtr(_) => 5,
        std::any::TypeId::Fn(_) => 6,
        std::any::TypeId::Usize => 7,
        _ => 99,
    }
}

fn id(x: i32) -> i32 {
    x + 1
}

fn main() {
    let cx: i32 = 9;
    let ax: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(11);

    let a = [1, 2, 3];

    let pa = &a;
    if pa[0] != 1 {
        println!("pa[0] != 1");
        std::process::exit(1);
    }
    if pa[1] != 2 {
        println!("pa[1] != 2");
        std::process::exit(2);
    }
    if pa[2] != 3 {
        println!("pa[2] != 3");
        std::process::exit(3);
    }

    if type_id(&a) != 99 {
        println!("type_id(&a) != 99");
        std::process::exit(4);
    }
    if type_id(&a[0]) != 2 {
        println!("type_id(&a[0]) != 2");
        std::process::exit(5);
    }

    if type_id(&cx) != 3 {
        println!("type_id(&cx) != 3");
        std::process::exit(6);
    }
    if type_id(cx) != 1 {
        println!("type_id(cx) != 1");
        std::process::exit(7);
    }
    if cx != 9 {
        println!("cx != 9");
        std::process::exit(8);
    }

    if type_id(ax) != 4 {
        println!("type_id(ax) != 4");
        std::process::exit(9);
    }
    if type_id(&ax) != 5 {
        println!("type_id(&ax) != 5");
        std::process::exit(10);
    }
    if type_id(ax) != 1 {
        println!("type_id(ax) != 1");
        std::process::exit(11);
    }
    if ax.load(std::sync::atomic::Ordering::SeqCst) != 11 {
        println!("ax.load() != 11");
        std::process::exit(12);
    }

    let fp: fn(i32) -> i32 = id;
    if type_id(fp) != 6 {
        println!("type_id(fp) != 6");
        std::process::exit(13);
    }
    if fp(4) != 5 {
        println!("fp(4) != 5");
        std::process::exit(14);
    }
    if id(4) != 5 {
        println!("id(4) != 5");
        std::process::exit(15);
    }

    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        println!("std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>()");
        std::process::exit(16);
    }
    if std::mem::align_of_val(&a[0]) != std::mem::align_of::<i32>() {
        println!("std::mem::align_of_val(&a[0]) != std::mem::align_of::<i32>()");
        std::process::exit(17);
    }

    println!("All tests passed");
    std::process::exit(0);
}