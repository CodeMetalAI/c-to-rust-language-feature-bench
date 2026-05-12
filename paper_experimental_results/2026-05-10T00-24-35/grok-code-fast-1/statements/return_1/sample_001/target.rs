#[derive(Clone, Copy)]
struct S {
    i: f64,
}

fn f(shared_i: &f64) -> S {
    S { i: *shared_i }
}

fn foo() -> i32 {
    1
}

fn main() -> i32 {
    let mut shared_i: f64 = 0.0;
    // g.u1.f2.i = 1.0;
    shared_i = 1.0;
    // g.u2.f3.i = 1.0;
    shared_i = 1.0;
    // g.u1.f2 = f();
    let s = f(&shared_i);
    shared_i = s.i;
    // g.u2.f3 = f();
    shared_i = s.i;
    let val: f64 = foo() as f64;
    let sum = shared_i + shared_i + val;
    if (sum == 3.0) { 0 } else { 1 }
}