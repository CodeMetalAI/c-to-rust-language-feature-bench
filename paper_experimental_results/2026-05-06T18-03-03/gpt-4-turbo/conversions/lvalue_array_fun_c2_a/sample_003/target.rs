fn main() {
    const CX: i32 = 9;
    let ax = std::sync::atomic::AtomicI32::new(11);

    let a = [1, 2, 3];
    let pa = &a;
    if pa[0] != 1 {
        std::process::exit(1);
    }
    if pa[1] != 2 {
        std::process::exit(2);
    }
    if pa[2] != 3 {
        std::process::exit(3);
    }

    // Rust does not have a direct equivalent to C's _Generic, so we use functions to mimic behavior
    if type_id_array(&a) != 99 {
        std::process::exit(4);
    }
    if type_id_int_pointer(&a[0]) != 2 {
        std::process::exit(5);
    }

    if type_id_const_int_pointer(&CX) != 3 {
        std::process::exit(6);
    }
    if type_id_int(CX) != 1 {
        std::process::exit(7);
    }
    if CX != 9 {
        std::process::exit(8);
    }

    if type_id_atomic_int(&ax) != 4 {
        std::process::exit(9);
    }
    if type_id_atomic_int_pointer(&ax) != 5 {
        std::process::exit(10);
    }
    let ax_val = ax.load(std::sync::atomic::Ordering::SeqCst);
    if type_id_int(ax_val) != 1 {
        std::process::exit(11);
    }
    if ax_val != 11 {
        std::process::exit(12);
    }

    let fp = id;
    if type_id_function_pointer(fp) != 6 {
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
}

fn id(v: i32) -> i32 {
    v + 1
}

fn type_id_array(_: &[i32; 3]) -> i32 {
    99
}

fn type_id_int_pointer(_: &i32) -> i32 {
    2
}

fn type_id_const_int_pointer(_: &i32) -> i32 {
    3
}

fn type_id_int(_: i32) -> i32 {
    1
}

fn type_id_atomic_int(_: &std::sync::atomic::AtomicI32) -> i32 {
    4
}

fn type_id_atomic_int_pointer(_: &std::sync::atomic::AtomicI32) -> i32 {
    5
}

fn type_id_function_pointer(_: fn(i32) -> i32) -> i32 {
    6
}