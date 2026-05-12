#[derive(Clone, Copy)]
struct S {
    i: f64,
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
    unsafe { G.u1_f2 }
}

fn foo() -> i32 {
    1
}

fn main() {
    unsafe {
        G.u1_f2.i = 1.0;
        G.u2_f3.i = 1.0;
        G.u1_f2 = f();
        G.u2_f3 = f();
    }
    let val = foo() as f64;
    let sum = unsafe { G.u1_f2.i + G.u2_f3.i } + val;
    std::process::exit(if sum == 3.0 { 0 } else { 1 });
}