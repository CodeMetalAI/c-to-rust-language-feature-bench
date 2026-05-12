#[derive(Clone, Copy)]
struct S {
    i: f64,
}

struct U1 {
    f1: i32,
    f2: S,
}

struct U2 {
    f3: S,
    f4: i32,
}

struct G {
    u1_f2: S,
    u2_f3: S,
}

static mut G: G = G {
    u1_f2: S { i: 0.0 },
    u2_f3: S { i: 0.0 },
};

fn f() -> S {
    // In the original C code, this returns g.u1.f2
    // We need to simulate this behavior
    S { i: unsafe { G.u1_f2.i } }
}

fn foo() -> i32 {
    1
}

fn main() {
    // g.u1.f2.i = 1.0;
    unsafe {
        G.u1_f2.i = 1.0;
    }
    // g.u2.f3.i = 1.0;
    unsafe {
        G.u2_f3.i = 1.0;
    }
    // g.u1.f2 = f();
    let result = f();
    unsafe {
        G.u1_f2 = result;
    }
    // g.u2.f3 = f();
    let result = f();
    unsafe {
        G.u2_f3 = result;
    }
    // double val = foo();
    let val: f64 = foo() as f64;
    // return g.u1.f2.i + g.u2.f3.i + val == 3.0 ? 0 : 1;
    let sum = unsafe { G.u1_f2.i + G.u2_f3.i + val };
    std::process::exit(if sum == 3.0 { 0 } else { 1 });
}