use std::mem;
use std::process::exit;
use std::sync::atomic::{AtomicI32, Ordering};

trait TypeId {
    const ID: i32;
}

fn type_id_of<T: TypeId>(_: &T) -> i32 {
    T::ID
}

impl TypeId for i32 {
    const ID: i32 = 1;
}
impl TypeId for *mut i32 {
    const ID: i32 = 2;
}
impl TypeId for *const i32 {
    const ID: i32 = 3;
}
impl TypeId for AtomicI32 {
    const ID: i32 = 4;
}
impl TypeId for *mut AtomicI32 {
    const ID: i32 = 5;
}
impl TypeId for fn(i32) -> i32 {
    const ID: i32 = 6;
}
impl TypeId for [i32; 3] {
    const ID: i32 = 99;
}

fn id(v: i32) -> i32 {
    v + 1
}

fn main() {
    let cx: i32 = 9;
    let mut ax = AtomicI32::new(11);

    let mut a = [1, 2, 3];

    {
        let pa: &[i32] = &a;
        if pa[0] != 1 {
            exit(1);
        }
        if pa[1] != 2 {
            exit(2);
        }
        if pa[2] != 3 {
            exit(3);
        }
    }

    if type_id_of(&a) != 99 {
        exit(4);
    }
    let a0_ptr: *mut i32 = &mut a[0];
    if type_id_of(&a0_ptr) != 2 {
        exit(5);
    }

    let cx_ptr: *const i32 = &cx;
    if type_id_of(&cx_ptr) != 3 {
        exit(6);
    }
    let cx_plus = cx;
    if type_id_of(&cx_plus) != 1 {
        exit(7);
    }
    if cx_plus != 9 {
        exit(8);
    }

    if type_id_of(&ax) != 4 {
        exit(9);
    }
    let ax_ptr: *mut AtomicI32 = &mut ax;
    if type_id_of(&ax_ptr) != 5 {
        exit(10);
    }
    let ax_val = ax.load(Ordering::SeqCst);
    if type_id_of(&ax_val) != 1 {
        exit(11);
    }
    if ax_val != 11 {
        exit(12);
    }

    let fp: fn(i32) -> i32 = id;
    if type_id_of(&fp) != 6 {
        exit(13);
    }
    if fp(4) != 5 {
        exit(14);
    }
    if id(4) != 5 {
        exit(15);
    }

    if mem::size_of::<[i32; 3]>() != 3 * mem::size_of::<i32>() {
        exit(16);
    }
    if mem::align_of::<i32>() != mem::align_of::<i32>() {
        exit(17);
    }
}