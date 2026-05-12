#[derive(Debug, Default)]
struct S {
    i: f64,
}

#[derive(Debug, Default)]
union U {
    u1: U1,
    u2: U2,
}

#[derive(Debug, Default)]
struct U1 {
    f1: i32,
    f2: S,
}

#[derive(Debug, Default)]
struct U2 {
    f3: S,
    f4: i32,
}

static mut G: U = U::default();

impl S {
    fn new() -> Self {
        S { i: 0.0 }
    }
}

fn f() -> S {
    unsafe { G.u1.f2 }
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
        let val = foo();
        std::process::exit(if G.u1.f2.i + G.u2.f3.i + val == 3.0 {
            0
        } else {
            1
        });
    }
}