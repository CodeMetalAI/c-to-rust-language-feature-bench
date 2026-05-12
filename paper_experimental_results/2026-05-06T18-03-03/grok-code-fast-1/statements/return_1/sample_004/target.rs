use std::cell::RefCell;

#[derive(Clone, Copy)]
struct S {
    i: f64,
}

struct G {
    u1_f2: S,
    u2_f3: S,
}

static G_VAR: RefCell<G> = RefCell::new(G {
    u1_f2: S { i: 0.0 },
    u2_f3: S { i: 0.0 },
});

fn f() -> S {
    G_VAR.borrow().u1_f2
}

fn foo() -> i32 {
    1
}

fn main() {
    G_VAR.borrow_mut().u1_f2.i = 1.0;
    G_VAR.borrow_mut().u2_f3.i = 1.0;
    G_VAR.borrow_mut().u1_f2 = f();
    G_VAR.borrow_mut().u2_f3 = f();
    let val = foo() as f64;
    let sum = G_VAR.borrow().u1_f2.i + G_VAR.borrow().u2_f3.i + val;
    std::process::exit(if sum == 3.0 { 0 } else { 1 });
}