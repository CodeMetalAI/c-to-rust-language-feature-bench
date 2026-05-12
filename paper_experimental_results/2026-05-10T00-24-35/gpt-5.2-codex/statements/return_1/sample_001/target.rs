use std::sync::Mutex;

#[derive(Copy, Clone)]
struct S {
    i: f64,
}

struct Global {
    u1_f2: S,
    u2_f3: S,
}

static G: Mutex<Global> = Mutex::new(Global {
    u1_f2: S { i: 0.0 },
    u2_f3: S { i: 0.0 },
});

fn f() -> S {
    let g = G.lock().unwrap();
    g.u1_f2
}

fn foo() -> i32 {
    1
}

fn main() {
    {
        let mut g = G.lock().unwrap();
        g.u1_f2.i = 1.0;
    }
    {
        let mut g = G.lock().unwrap();
        g.u2_f3.i = 1.0;
    }
    {
        let val = f();
        let mut g = G.lock().unwrap();
        g.u1_f2 = val;
    }
    {
        let val = f();
        let mut g = G.lock().unwrap();
        g.u2_f3 = val;
    }
    let val = foo() as f64;
    let g = G.lock().unwrap();
    let sum = g.u1_f2.i + g.u2_f3.i + val;
    let code = if sum == 3.0 { 0 } else { 1 };
    std::process::exit(code);
}