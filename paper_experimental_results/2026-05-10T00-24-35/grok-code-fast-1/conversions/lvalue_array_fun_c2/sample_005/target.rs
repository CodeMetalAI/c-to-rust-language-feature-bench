use std::sync::atomic::{AtomicI32, Ordering};

fn type_id(_: i32) -> i32 { 1 }
fn type_id_atomic(_: &AtomicI32) -> i32 { 3 }

fn main() {
    let mut x: i32 = 3;
    const CX: i32 = 4;
    let ax = AtomicI32::new(5);

    if std::mem::size_of_val(&x) != std::mem::size_of::<i32>() {
        std::process::exit(1);
    }
    if std::mem::align_of_val(&x) != std::mem::align_of::<i32>() {
        std::process::exit(2);
    }

    if *(&x) != 3 {
        std::process::exit(3);
    }

    if type_id(x) != 1 {
        std::process::exit(4);
    }
    if type_id_atomic(&ax) != 3 {
        std::process::exit(5);
    }

    if type_id(CX) != 1 {
        std::process::exit(6);
    }
    if type_id(ax.load(Ordering::SeqCst)) != 1 {
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

    #[derive(Clone, Copy)]
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