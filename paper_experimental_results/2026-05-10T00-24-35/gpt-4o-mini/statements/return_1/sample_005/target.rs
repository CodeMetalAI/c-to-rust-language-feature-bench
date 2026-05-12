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

fn foo() -> i32 {
    1
}

fn main() -> i32 {
    unsafe {
        G.u1.1.i = 1.0;
        G.u2.0.i = 1.0;
        G.u1.1 = f();
        G.u2.0 = f();
    }
    let val = foo();
    if unsafe { G.u1.1.i + G.u2.0.i + val as f64 == 3.0 } {
        0
    } else {
        1
    }
}