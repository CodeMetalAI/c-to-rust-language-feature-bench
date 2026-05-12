use std::sync::{Mutex, OnceLock};

#[derive(Copy, Clone, Default)]
struct S {
    i: f64,
}

#[derive(Copy, Clone, Default)]
struct U {
    f1: i32,
    u1_f2: S,
    u2_f3: S,
    f4: i32,
}

fn get_g() -> &'static Mutex<U> {
    static G: OnceLock<Mutex<U>> = OnceLock::new();
    G.get_or_init(|| Mutex::new(U::default()))
}

fn f() -> S {
    let g = get_g();
    let guard = g.lock().unwrap();
    guard.u1_f2
}

fn foo() -> i32 {
    1
}

fn main() {
    {
        let g = get_g();
        let mut guard = g.lock().unwrap();
        guard.u1_f2.i = 1.0;
    }
    {
        let g = get_g();
        let mut guard = g.lock().unwrap();
        guard.u2_f3.i = 1.0;
    }

    let val_f = f();
    {
        let g = get_g();
        let mut guard = g.lock().unwrap();
        guard.u1_f2 = val_f;
    }

    let val_f = f();
    {
        let g = get_g();
        let mut guard = g.lock().unwrap();
        guard.u2_f3 = val_f;
    }

    let val = foo() as f64;
    let (a, b) = {
        let g = get_g();
        let guard = g.lock().unwrap();
        (guard.u1_f2.i, guard.u2_f3.i)
    };

    let ret = if a + b + val == 3.0 { 0 } else { 1 };
    std::process::exit(ret);
}