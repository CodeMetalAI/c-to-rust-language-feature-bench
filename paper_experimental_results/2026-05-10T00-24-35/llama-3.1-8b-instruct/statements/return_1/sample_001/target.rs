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

static mut G: U = U { u1: struct { f1: 0, f2: S { i: 0.0 } } };

fn f() -> S {
    G.u1.f2
}

fn foo() -> f64 {
    1.0
}

fn main() {
    unsafe {
        G.u1.f2.i = 1.0;
        G.u2.f3.i = 1.0;
        G.u1.f2 = f();
        G.u2.f3 = f();
    }
    let val = foo();
    let result = (G.u1.f2.i + G.u2.f3.i + val) == 3.0;
    std::process::exit(if result { 0 } else { 1 });
}