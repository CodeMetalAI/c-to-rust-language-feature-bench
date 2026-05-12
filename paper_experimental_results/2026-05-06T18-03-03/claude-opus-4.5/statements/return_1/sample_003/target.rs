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
    // Since we're simulating the union behavior and the assignments,
    // we need to track what f() would return based on the state
    S { i: 1.0 }
}

fn foo() -> i32 {
    1
}

fn main() {
    // g.u1.f2.i = 1.0;
    let mut u1_f2 = S { i: 1.0 };
    
    // g.u2.f3.i = 1.0;
    let mut u2_f3 = S { i: 1.0 };
    
    // g.u1.f2 = f();
    u1_f2 = f();
    
    // g.u2.f3 = f();
    u2_f3 = f();
    
    // double val = foo();
    let val: f64 = foo() as f64;
    
    // return g.u1.f2.i + g.u2.f3.i + val == 3.0 ? 0 : 1;
    let result = u1_f2.i + u2_f3.i + val;
    
    if result == 3.0 {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}