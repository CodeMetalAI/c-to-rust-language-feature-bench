use std::mem;
use std::sync::atomic::{AtomicI32, Ordering};

fn main() {
    let mut x: i32 = 3;
    let cx: i32 = 4;
    let ax = AtomicI32::new(5);

    if mem::size_of_val(&x) != mem::size_of::<i32>() {
        std::process::exit(1);
    }
    if mem::align_of_val(&x) != mem::align_of::<i32>() {
        std::process::exit(2);
    }

    if *(&x) != 3 {
        std::process::exit(3);
    }

    macro_rules! TYPE_ID {
        ($e:expr) => {
            match std::any::type_name_of_val(&$e) {
                "i32" => 1,
                "std::sync::atomic::AtomicI32" => 3,
                _ => 99,
            }
        };
    }

    if TYPE_ID!(x) != 1 {
        std::process::exit(4);
    }
    if TYPE_ID!(ax) != 3 {
        std::process::exit(5);
    }

    if TYPE_ID!(cx) != 1 {
        std::process::exit(6);
    }
    if TYPE_ID!(ax.load(Ordering::SeqCst)) != 1 {
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

    std::process::exit(0);
}