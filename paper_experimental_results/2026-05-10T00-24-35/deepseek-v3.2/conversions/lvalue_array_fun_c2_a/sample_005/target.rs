use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering;

const TYPE_ID_INT: u32 = 1;
const TYPE_ID_INT_PTR: u32 = 2;
const TYPE_ID_CONST_INT_PTR: u32 = 3;
const TYPE_ID_ATOMIC_INT: u32 = 4;
const TYPE_ID_ATOMIC_INT_PTR: u32 = 5;
const TYPE_ID_FN_PTR: u32 = 6;
const TYPE_ID_CONST_INT: u32 = 7;
const TYPE_ID_DEFAULT: u32 = 99;

fn id(v: i32) -> i32 {
    v + 1
}

fn type_id_int(_: i32) -> u32 {
    TYPE_ID_INT
}

fn type_id_int_ptr(_: *const i32) -> u32 {
    TYPE_ID_INT_PTR
}

fn type_id_const_int_ptr(_: *const i32) -> u32 {
    TYPE_ID_CONST_INT_PTR
}

fn type_id_atomic_int(_: &AtomicI32) -> u32 {
    TYPE_ID_ATOMIC_INT
}

fn type_id_atomic_int_ptr(_: *const AtomicI32) -> u32 {
    TYPE_ID_ATOMIC_INT_PTR
}

fn type_id_fn_ptr(_: fn(i32) -> i32) -> u32 {
    TYPE_ID_FN_PTR
}

fn type_id_const_int(_: i32) -> u32 {
    TYPE_ID_CONST_INT
}

fn type_id_default<T>(_: T) -> u32 {
    TYPE_ID_DEFAULT
}

fn main() {
    const CX: i32 = 9;
    let ax = AtomicI32::new(11);

    let a = [1, 2, 3];

    let pa: *const i32 = a.as_ptr();
    unsafe {
        if *pa.offset(0) != 1 {
            std::process::exit(1);
        }
        if *pa.offset(1) != 2 {
            std::process::exit(2);
        }
        if *pa.offset(2) != 3 {
            std::process::exit(3);
        }
    }

    if type_id_default(a) != TYPE_ID_DEFAULT {
        std::process::exit(4);
    }
    if type_id_int_ptr(&a[0] as *const i32) != TYPE_ID_INT_PTR {
        std::process::exit(5);
    }

    if type_id_const_int_ptr(&CX as *const i32) != TYPE_ID_CONST_INT_PTR {
        std::process::exit(6);
    }
    if type_id_int(CX) != TYPE_ID_INT {
        std::process::exit(7);
    }
    if CX != 9 {
        std::process::exit(8);
    }

    if type_id_atomic_int(&ax) != TYPE_ID_ATOMIC_INT {
        std::process::exit(9);
    }
    if type_id_atomic_int_ptr(&ax as *const AtomicI32) != TYPE_ID_ATOMIC_INT_PTR {
        std::process::exit(10);
    }
    if type_id_int(ax.load(Ordering::SeqCst)) != TYPE_ID_INT {
        std::process::exit(11);
    }
    if ax.load(Ordering::SeqCst) != 11 {
        std::process::exit(12);
    }

    let fp: fn(i32) -> i32 = id;
    if type_id_fn_ptr(id) != TYPE_ID_FN_PTR {
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
    if std::mem::align_of_val(&a[0]) != std::mem::align_of::<i32>() {
        std::process::exit(17);
    }

    std::process::exit(0);
}