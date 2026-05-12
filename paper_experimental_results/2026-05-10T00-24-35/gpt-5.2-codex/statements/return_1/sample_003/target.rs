use std::cell::RefCell;
use std::process;

#[derive(Copy, Clone)]
struct S {
    i: f64,
}

struct G {
    u1_f2: S,
    u2_f3: S,
}

thread_local! {
    static GLOB: RefCell<G> = RefCell::new(G {
        u1_f2: S { i: 0.0 },
        u2_f3: S { i: 0.0 },
    });
}

fn f() -> S {
    GLOB.with(|g| g.borrow().u1_f2)
}

fn foo() -> i32 {
    1
}

fn main() {
    GLOB.with(|g| {
        g.borrow_mut().u1_f2.i = 1.0;
    });
    GLOB.with(|g| {
        g.borrow_mut().u2_f3.i = 1.0;
    });

    let tmp = f();
    GLOB.with(|g| {
        g.borrow_mut().u1_f2 = tmp;
    });

    let tmp2 = f();
    GLOB.with(|g| {
        g.borrow_mut().u2_f3 = tmp2;
    });

    let val = foo() as f64;
    let (a, b) = GLOB.with(|g| {
        let g = g.borrow();
        (g.u1_f2.i, g.u2_f3.i)
    });

    let sum = a + b + val;
    let code = if sum == 3.0 { 0 } else { 1 };
    process::exit(code);
}