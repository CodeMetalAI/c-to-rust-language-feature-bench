#[derive(Debug, Clone, Copy)]
struct S {
    i: f64,
}

#[derive(Debug, Clone, Copy)]
union U {
    u1: U1,
    u2: U2,
}

#[derive(Debug, Clone, Copy)]
struct U1 {
    f1: i32,
    f2: S,
}

#[derive(Debug, Clone, Copy)]
struct U2 {
    f3: S,
    f4: i32,
}

static mut G: U = unsafe { U { u1: U1 { f1: 0, f2: S { i: 0.0 } } } };

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
    }
    let val = foo() as f64;
    let exit_code = if unsafe { G.u1.f2.i + G.u2.f3.i + val == 3.0 } { 0 } else { 1 };
    std::process::exit(exit_code);
}