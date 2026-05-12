struct S {
    i: f64,
}

union U {
    u1: U1,
    u2: U2,
}

#[derive(Copy, Clone)]
struct U1 {
    f1: i32,
    f2: S,
}

#[derive(Copy, Clone)]
struct U2 {
    f3: S,
    f4: i32,
}

static mut G: U = U {
    u1: U1 {
        f1: 0,
        f2: S { i: 0.0 },
    },
};

fn f() -> S {
    unsafe { G.u1.f2 }
}

fn foo() -> i32 {
    1
}

fn main() {
    unsafe {
        G.u1.f2.i = 1.0;
        G.u2.f3.i = 1.0;
        G.u1.f2 = f();
        G.u2.f3 = f();
        let val = foo() as f64;
        std::process::exit(if G.u1.f2.i + G.u2.f3.i + val == 3.0 { 0 } else { 1 });
    }
}