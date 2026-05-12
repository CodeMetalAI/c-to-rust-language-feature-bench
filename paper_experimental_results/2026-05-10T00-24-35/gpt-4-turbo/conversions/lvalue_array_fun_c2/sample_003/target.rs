fn main() {
    let mut x = 3;
    let cx = 4;
    let mut ax = 5;

    if std::mem::size_of_val(&x) != std::mem::size_of::<i32>() {
        return std::process::exit(1);
    }
    if std::mem::align_of_val(&x) != std::mem::align_of::<i32>() {
        return std::process::exit(2);
    }

    if *(&x) != 3 {
        return std::process::exit(3);
    }

    if type_id(x) != 1 {
        return std::process::exit(4);
    }
    if type_id(ax) != 3 {
        return std::process::exit(5);
    }

    // Note: Rust does not have a concept of `_Atomic` in safe Rust,
    // so we treat `ax` as a regular integer for the purpose of this mimic.
    if type_id(cx) != 1 {
        return std::process::exit(6);
    }
    if type_id(ax) != 1 {
        return std::process::exit(7);
    }

    let y = x;
    if y != 3 {
        return std::process::exit(8);
    }

    x += 1;
    if x != 4 {
        return std::process::exit(9);
    }

    let mut s = S { m: 7 };
    if s.m != 7 {
        return std::process::exit(10);
    }
    s.m = 8;
    if s.m != 8 {
        return std::process::exit(11);
    }
}

struct S {
    m: i32,
}

fn type_id<T>(_: T) -> i32 {
    match std::any::type_name::<T>() {
        "i32" => 1,
        "atomic::AtomicI32" => 3, // Simulating _Atomic(int) in Rust
        _ => 99,
    }
}