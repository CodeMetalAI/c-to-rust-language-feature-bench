struct S {
    i: f64,
}

union U {
    u1: S,
    u2: S,
}

fn f() -> S {
    U { u1: S { i: 0.0 } }.u1
}

fn foo() -> i32 {
    1
}

fn main() {
    let mut g = U { u1: S { i: 1.0 } };
    g.u1.i = 1.0;
    g.u2.i = 1.0;
    g.u1 = f();
    g.u2 = f();
    let val = foo() as f64;
    if g.u1.i + g.u2.i + val == 3.0 {
        0
    } else {
        1
    }
}