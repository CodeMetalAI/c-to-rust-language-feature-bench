struct S {
    i: f64,
}

union U {
    struct {
        f1: i32,
        f2: S,
    } u1,
    struct {
        f3: S,
        f4: i32,
    } u2,
}

static mut G: U = U { u1: f2: S { i: 0.0 } };

fn f() -> S {
    S { i: 0.0 }
}

fn foo() -> f64 {
    1.0
}

fn main() {
    G.u1.f2.i = 1.0;
    G.u2.f3.i = 1.0;
    G.u1.f2 = f();
    G.u2.f3 = f();
    let val: f64 = foo();
    assert_eq!(G.u1.f2.i + G.u2.f3.i + val, 3.0);
}