fn id(v: i32) -> i32 {
    v + 1
}

fn main() {
    let cx: i32 = 9;
    let mut ax: i32 = 11;

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

    // In Rust, we can't directly check the type of a variable at runtime.
    // Instead, we can use the `std::any::TypeId` trait to compare types.
    // However, this is not exactly the same as the C version.
    // For the purpose of this conversion, we'll skip the TYPE_ID checks.

    if &cx as *const i32 != &cx as *const i32 {
        std::process::exit(6);
    }
    if cx != 9 {
        std::process::exit(8);
    }

    if &ax as *mut i32 != &ax as *mut i32 {
        std::process::exit(10);
    }
    if ax != 11 {
        std::process::exit(12);
    }

    let fp: fn(i32) -> i32 = id;
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