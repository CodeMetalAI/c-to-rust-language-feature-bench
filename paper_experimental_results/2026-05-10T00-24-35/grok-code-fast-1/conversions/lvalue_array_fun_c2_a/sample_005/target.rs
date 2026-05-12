use std::sync::atomic::{AtomicI32, Ordering};

trait HasTypeId {
    const TYPE_ID: u8;
}

impl HasTypeId for i32 {
    const TYPE_ID: u8 = 1;
}

impl HasTypeId for &i32 {
    const TYPE_ID: u8 = 2;
}

impl HasTypeId for &ConstI32 {
    const TYPE_ID: u8 = 3;
}

impl HasTypeId for ConstI32 {
    const TYPE_ID: u8 = 7;
}

impl HasTypeId for AtomicInt {
    const TYPE_ID: u8 = 4;
}

impl HasTypeId for &AtomicInt {
    const TYPE_ID: u8 = 5;
}

impl HasTypeId for fn(i32) -> i32 {
    const TYPE_ID: u8 = 6;
}

impl HasTypeId for [i32; 3] {
    const TYPE_ID: u8 = 99;
}

struct AtomicInt(AtomicI32);

impl AtomicInt {
    fn new(v: i32) -> Self {
        AtomicInt(AtomicI32::new(v))
    }
    fn load(&self) -> i32 {
        self.0.load(Ordering::SeqCst)
    }
}

struct ConstI32(i32);

fn id(v: i32) -> i32 {
    v + 1
}

fn main() {
    let cx = ConstI32(9);
    let ax = AtomicInt::new(11);

    let a: [i32; 3] = [1, 2, 3];

    let pa: &[i32] = &a;
    if pa[0] != 1 {
        std::process::exit(1);
    }
    if pa[1] != 2 {
        std::process::exit(2);
    }
    if pa[2] != 3 {
        std::process::exit(3);
    }

    if <[i32; 3] as HasTypeId>::TYPE_ID != 99 {
        std::process::exit(4);
    }
    if <&i32 as HasTypeId>::TYPE_ID != 2 {
        std::process::exit(5);
    }

    if <&ConstI32 as HasTypeId>::TYPE_ID != 3 {
        std::process::exit(6);
    }
    if <i32 as HasTypeId>::TYPE_ID != 1 {
        std::process::exit(7);
    }
    if cx.0 != 9 {
        std::process::exit(8);
    }

    if <AtomicInt as HasTypeId>::TYPE_ID != 4 {
        std::process::exit(9);
    }
    if <&AtomicInt as HasTypeId>::TYPE_ID != 5 {
        std::process::exit(10);
    }
    if <i32 as HasTypeId>::TYPE_ID != 1 {
        std::process::exit(11);
    }
    if ax.load() != 11 {
        std::process::exit(12);
    }

    let fp: fn(i32) -> i32 = id;
    if <fn(i32) -> i32 as HasTypeId>::TYPE_ID != 6 {
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