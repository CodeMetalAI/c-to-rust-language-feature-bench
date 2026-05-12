use std::sync::atomic::AtomicI32;

fn id(v: i32) -> i32 {
    v + 1
}

fn type_id<T>(_: &T) -> u32 {
    std::any::TypeId::of::<T>().hash(&mut 0u64) as u32
}

fn main() {
    const CX: i32 = 9;
    let ax = AtomicI32::new(11);

    let a = [1, 2, 3];

    let pa = &a[0] as *const i32;
    unsafe {
        if *pa != 1 {
            std::process::exit(1);
        }
        if *pa.offset(1) != 2 {
            std::process::exit(2);
        }
        if *pa.offset(2) != 3 {
            std::process::exit(3);
        }
    }

    // TYPE_ID(a) - array type, not in our mapping, treat as 99
    if type_id(&a) != type_id(&99u32) {
        std::process::exit(4);
    }

    // TYPE_ID(&a[0]) - pointer to int
    if type_id(&(&a[0] as *const i32)) != type_id(&(&0i32 as *const i32)) {
        std::process::exit(5);
    }

    // TYPE_ID(&cx) - pointer to const int
    if type_id(&(&CX as *const i32)) != type_id(&(&0i32 as *const i32)) {
        std::process::exit(6);
    }

    // TYPE_ID(+cx) - int (promoted)
    if type_id(&CX) != type_id(&0i32) {
        std::process::exit(7);
    }
    if CX != 9 {
        std::process::exit(8);
    }

    // TYPE_ID(ax) - AtomicI32
    if type_id(&ax) != type_id(&AtomicI32::new(0)) {
        std::process::exit(9);
    }

    // TYPE_ID(&ax) - pointer to AtomicI32
    if type_id(&(&ax as *const AtomicI32)) != type_id(&(&AtomicI32::new(0) as *const AtomicI32)) {
        std::process::exit(10);
    }

    // TYPE_ID(+ax) - int (load atomic)
    let ax_val = ax.load(std::sync::atomic::Ordering::SeqCst);
    if type_id(&ax_val) != type_id(&0i32) {
        std::process::exit(11);
    }
    if ax_val != 11 {
        std::process::exit(12);
    }

    // TYPE_ID(id) - function pointer
    let fp: fn(i32) -> i32 = id;
    if type_id(&fp) != type_id(&(id as fn(i32) -> i32)) {
        std::process::exit(13);
    }
    if fp(4) != 5 {
        std::process::exit(14);
    }
    if id(4) != 5 {
        std::process::exit(15);
    }

    // sizeof a
    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        std::process::exit(16);
    }

    // alignof
    if std::mem::align_of::<i32>() != std::mem::align_of_val(&a[0]) {
        std::process::exit(17);
    }

    std::process::exit(0);
}