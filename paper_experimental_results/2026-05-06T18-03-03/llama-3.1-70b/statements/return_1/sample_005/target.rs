#[derive(Debug, Copy, Clone)]
struct S {
    i: f64,
}

#[derive(Debug, Copy, Clone)]
union U {
    u1: (i32, S),
    u2: (S, i32),
}

fn f() -> S {
    let mut g: U = unsafe { std::mem::zeroed() };
    g.u1.1
}

fn foo() -> f64 {
    1.0
}

fn main() {
    let mut g: U = unsafe { std::mem::zeroed() };
    unsafe {
        g.u1.1.i = 1.0;
        g.u2.0.i = 1.0;
        g.u1.1 = f();
        g.u2.0 = f();
    }
    let val = foo();
    if (g.u1.1.i + g.u2.0.i + val) == 3.0 {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}