use std::cell::RefCell;
use std::thread_local;

#[derive(Clone, Copy, Default)]
struct S {
    i: f64,
}

#[derive(Clone, Copy, Default)]
struct U1 {
    f1: i32,
    f2: S,
}

#[derive(Clone, Copy, Default)]
struct U2 {
    f3: S,
    f4: i32,
}

#[derive(Clone, Copy, Default)]
struct U {
    u1: U1,
    u2: U2,
}

thread_local! {
    static G: RefCell<U> = RefCell::new(U::default());
}

fn f() -> S {
    G.with(|g| g.borrow().u1.f2)
}

fn foo() -> i32 {
    1
}

fn main() {
    G.with(|g| {
        let mut g = g.borrow_mut();
        g.u1.f2.i = 1.0;
        g.u2.f3.i = 1.0;
        g.u1.f2 = f();
        g.u2.f3 = f();
        let val = foo() as f64;
        let sum = g.u1.f2.i + g.u2.f3.i + val;
        std::process::exit(if sum == 3.0 { 0 } else { 1 });
    });
}