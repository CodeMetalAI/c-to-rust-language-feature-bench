use std::sync::atomic::{AtomicI32, Ordering};

struct ConstInt(i32);

impl ConstInt {
    fn value(&self) -> i32 {
        self.0
    }
}

trait TypeId {
    fn type_id(&self) -> i32;
}

impl TypeId for [i32; 3] {
    fn type_id(&self) -> i32 {
        99
    }
}

impl TypeId for &i32 {
    fn type_id(&self) -> i32 {
        2
    }
}

impl TypeId for &ConstInt {
    fn type_id(&self) -> i32 {
        3
    }
}

impl TypeId for i32 {
    fn type_id(&self) -> i32 {
        1
    }
}

impl TypeId for AtomicI32 {
    fn type_id(&self) -> i32 {
        4
    }
}

impl TypeId for &AtomicI32 {
    fn type_id(&self) -> i32 {
        5
    }
}

impl TypeId for fn(i32) -> i32 {
    fn type_id(&self) -> i32 {
        6
    }
}

impl TypeId for ConstInt {
    fn type_id(&self) -> i32 {
        7
    }
}

fn id(v: i32) -> i32 {
    v + 1
}

fn main() {
    let cx = ConstInt(9);
    let ax = AtomicI32::new(11);

    let a: [i32; 3] = [1, 2, 3];

    let pa = &a[..];
    if pa[0] != 1 {
        std::process::exit(1);
    }
    if pa[1] != 2 {
        std::process::exit(2);
    }
    if pa[2] != 3 {
        std::process::exit(3);
    }

    if a.type_id() != 99 {
        std::process::exit(4);
    }
    if (&a[0]).type_id() != 2 {
        std::process::exit(5);
    }

    if (&cx).type_id() != 3 {
        std::process::exit(6);
    }
    if cx.value().type_id() != 1 {
        std::process::exit(7);
    }
    if cx.value() != 9 {
        std::process::exit(8);
    }

    if ax.type_id() != 4 {
        std::process::exit(9);
    }
    if (&ax).type_id() != 5 {
        std::process::exit(10);
    }
    if ax.load(Ordering::SeqCst).type_id() != 1 {
        std::process::exit(11);
    }
    if ax.load(Ordering::SeqCst) != 11 {
        std::process::exit(12);
    }

    let fp: fn(i32) -> i32 = id;
    if id.type_id() != 6 {
        std::process::exit(13);
    }
    if fp(4) != 5 {
        std::process::exit(14);
    }
    if id(4) != 5 {
        std::process::exit(15);
    }

    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        std::process::exit(16);
    }
    if std::mem::align_of::<i32>() != std::mem::align_of::<i32>() {
        std::process::exit(17);
    }

    std::process::exit(0);
}