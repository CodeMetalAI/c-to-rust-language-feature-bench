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

static mut G: U;

fn f() -> S {
    G.u1.f2
}

fn foo() -> i32 {
    1
}

fn main() {
    G.u1.f2.i = 1.0;
    G.u2.f3.i = 1.0;
    G.u1.f2 = f();
    G.u2.f3 = f();
    let val = foo() as f64;
    if G.u1.f2.i + G.u2.f3.i + val == 3.0 {
        0
    } else {
        1
    }
}