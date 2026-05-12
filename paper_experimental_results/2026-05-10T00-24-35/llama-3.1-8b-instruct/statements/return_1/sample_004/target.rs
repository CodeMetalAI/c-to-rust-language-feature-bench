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
    g.u1.f2 = f();
    g.u2.f3 = f();
    let val = foo() as f64;
    assert_eq!(g.u1.f2.i + g.u2.f3.i + val, 3.0);
    std::process::exit(match g.u1.f2.i + g.u2.f3.i + val == 3.0 {
        true => 0,
        false => 1,
    });
}