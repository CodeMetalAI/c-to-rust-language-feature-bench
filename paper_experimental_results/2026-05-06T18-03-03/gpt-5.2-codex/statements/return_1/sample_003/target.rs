use std::cell::RefCell;

#[derive(Clone, Copy)]
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
        let mut g = g.borrow_mut();
        g.u1_f2.i = 1.0;
        g.u2_f3.i = 1.0;
    });

    let s1 = f();
    GLOB.with(|g| {
        g.borrow_mut().u1_f2 = s1;
    });

    let s2 = f();
    GLOB.with(|g| {
        g.borrow_mut().u2_f3 = s2;
    });

    let val = foo() as f64;
    let sum = GLOB.with(|g| {
        let g = g.borrow();
        g.u1_f2.i + g.u2_f3.i + val
    });

    let code = if (sum - 3.0).abs() < f64::EPSILON { 0 } else { 1 };
    std::process::exit(code);
}