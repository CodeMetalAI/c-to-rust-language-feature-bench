fn id(v: i32) -> i32 {
    v + 1
}

fn main() {
    let cx: i32 = 9;
    let mut ax: i32 = 11;

    let a: [i32; 3] = [1, 2, 3];

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

    // Note: Rust doesn't have a direct equivalent to _Generic in C/C++
    // So we can't directly translate TYPE_ID macro to Rust
    // Instead, we'll use type checking at compile time

    // Check types at compile time
    let _: &[i32; 3] = &a;
    let _: &i32 = &a[0];
    let _: &i32 = &cx;
    let _: i32 = cx;
    let _: &i32 = &ax;
    let _: i32 = ax;
    let _: fn(i32) -> i32 = id;

    if cx != 9 {
        std::process::exit(8);
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