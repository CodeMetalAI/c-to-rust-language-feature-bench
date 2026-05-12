use std::sync::atomic::{AtomicI32, Ordering};

macro_rules! type_id {
    ($x:expr) => {
        match $x {
            &i => 1,
            &mut i => 2,
            &i_const => 3,
            &atomic_i => 4,
            &mut atomic_i => 5,
            &func: fn(i32) -> i32 => 6,
            i_const => 7,
            _ => 99,
        }
    };
}

fn id(v: i32) -> i32 {
    v + 1
}

fn main() {
    const cx: i32 = 9;
    let ax = AtomicI32::new(11);

    let a = [1, 2, 3];

    let pa = a.as_ptr();
    if pa.add(0).read() != 1 {
        std::process::exit(1);
    }
    if pa.add(1).read() != 2 {
        std::process::exit(2);
    }
    if pa.add(2).read() != 3 {
        std::process::exit(3);
    }

    if type_id(&a) != 99 {
        std::process::exit(4);
    }
    if type_id(&a[0]) != 2 {
        std::process::exit(5);
    }

    if type_id(&cx) != 3 {
        std::process::exit(6);
    }
    if type_id(&cx + 0) != 1 {
        std::process::exit(7);
    }
    if cx != 9 {
        std::process::exit(8);
    }

    if type_id(&ax) != 4 {
        std::process::exit(9);
    }
    if type_id(&ax) != 5 {
        std::process::exit(10);
    }
    if type_id(&ax.load(Ordering::Relaxed)) != 1 {
        std::process::exit(11);
    }
    if ax.load(Ordering::Relaxed) != 11 {
        std::process::exit(12);
    }

    let fp: fn(i32) -> i32 = id;
    if type_id(&id) != 6 {
        std::process::exit(13);
    }
    if fp(4) != 5 {
        std::process::exit(14);
    }
    if id(4) != 5 {
        std::process::exit(15);
    }

    if std::mem::size_of::<[i32; 3]>() != 3 * std::mem::size_of::<i32>() {
        std::process::exit(16);
    }
    if std::mem::align_of::<i32>() != std::mem::align_of::<i32>() {
        std::process::exit(17);
    }

    std::process::exit(0);
}