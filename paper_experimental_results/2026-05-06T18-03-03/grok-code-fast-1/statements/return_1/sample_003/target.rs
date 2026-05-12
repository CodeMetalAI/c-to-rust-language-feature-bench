use std::cell::RefCell;

#[derive(Clone, Copy)]
struct S {
    i: f64,
}

fn f() -> S {
    G.borrow().clone()
}

fn foo() -> i32 {
    1
}

lazy_static::lazy_static! {
    static ref G: RefCell<S> = RefCell::new(S { i: 0.0 });
}

fn main() {
    G.borrow_mut().i = 1.0;
    G.borrow_mut().i = 1.0;
    *G.borrow_mut() = f();
    *G.borrow_mut() = f();
    let val = foo() as f64;
    let gi = G.borrow().i;
    std::process::exit(if gi + gi + val == 3.0 { 0 } else { 1 });
}