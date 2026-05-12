fn main() {
    use std::mem::{align_of, size_of};

    let mut x: i32 = 3;
    let cx: i32 = 4;
    let ax: i32 = 5; // No direct equivalent for _Atomic(int) in safe Rust

    if size_of::<i32>() != size_of_val(&x) {
        std::process::exit(1);
    }
    if align_of::<i32>() != align_of_val(&x) {
        std::process::exit(2);
    }

    if *(&x) != 3 {
        std::process::exit(3);
    }

    if type_id(&x) != 1 {
        std::process::exit(4);
    }
    if type_id(&ax) != 3 {
        std::process::exit(5);
    }

    if type_id(&(+cx)) != 1 {
        std::process::exit(6);
    }
    if type_id(&(+ax)) != 1 {
        std::process::exit(7);
    }

    let y = x;
    if y != 3 {
        std::process::exit(8);
    }

    x += 1;
    if x != 4 {
        std::process::exit(9);
    }

    struct S {
        m: i32,
    }

    let mut s = S { m: 7 };
    if s.m != 7 {
        std::process::exit(10);
    }
    s.m = 8;
    if s.m != 8 {
        std::process::exit(11);
    }
}

fn type_id<T>(_value: &T) -> usize {
    // Simulated type ID function
    std::any::TypeId::of::<T>() as *const _ as usize
}