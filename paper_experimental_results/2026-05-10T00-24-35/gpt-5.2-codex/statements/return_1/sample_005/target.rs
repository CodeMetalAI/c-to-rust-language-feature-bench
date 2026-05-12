use std::cell::RefCell;
use std::process::exit;

#[derive(Copy, Clone, Default)]
struct S {
    i: f64,
}

#[derive(Copy, Clone, Default)]
struct U {
    u1_f2: S,
    u2_f3: S,
}

thread_local! {
    static G: RefCell<U> = RefCell::new(U::default());
}

fn f() -> S {
    G.with(|g| g.borrow().u1_f2)
}

fn foo() -> i32 {
    1
}

fn main() {
    G.with(|g| {
        let mut g = g.borrow_mut();
        g.u1_f2.i = 1.0;
        g.u2_f3.i = 1.0;
    });

    let s1 = f();
    G.with(|g| g.borrow_mut().u1_f2 = s1);

    let s2 = f();
    G.with(|g| g.borrow_mut().u2_f3 = s2);

    let val = foo() as f64;
    let sum = G.with(|g| {
        let g = g.borrow();
        g.u1_f2.i + g.u2_f3.i + val
    });

    if sum == 3.0 {
        exit(0);
    } else {
        exit(1);
    }
}