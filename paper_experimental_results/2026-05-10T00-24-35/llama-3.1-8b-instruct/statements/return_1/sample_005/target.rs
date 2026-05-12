struct S {
    i: f64,
}

union U {
    u1: struct {
        f1: i32,
        f2: S,
    },
    u2: struct {
        f3: S,
        f4: i32,
    },
}

fn f() -> S {
    U { u1: S { i: 1.0 } }
}

fn foo() -> i32 {
    1
}

fn main() {
    let mut g = U { u1: S { i: 1.0 } };
    g.u1.f2.i = 1.0;
    g.u2.f3.i = 1.0;
    g.u1.f2 = f();
    g.u2.f3 = f();
    let val: f64 = (g.u1.f2.i as f64) as i32 as f64 + (g.u2.f3.i as f64) as i32 as f64 + foo() as f64;
    assert!(val == 3.0, "Test failed");
    std::process::exit(if val == 3.0 { 0 } else { 1 });
}