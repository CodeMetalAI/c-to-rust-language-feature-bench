struct S {
    i: f64,
}

union U {
    u1: (i32, S),
    u2: (S, i32),
}

static mut G: U = U { u1: (0, S { i: 0.0 }) };

fn f() -> S {
    unsafe { G.u1.1 }
}

fn foo() -> f64 {
    1.0
}

fn main() {
    unsafe {
        G.u1.1.i = 1.0;
        G.u2.0.i = 1.0;
        G.u1.1 = f();
        G.u2.0 = f();
    }
    let val = foo();
    if unsafe { G.u1.1.i + G.u2.0.i + val } == 3.0 {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}