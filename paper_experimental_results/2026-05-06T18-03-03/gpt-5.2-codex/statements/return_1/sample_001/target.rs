use std::sync::{Mutex, OnceLock};

#[derive(Copy, Clone)]
struct S {
    i: f64,
}

struct UnionU {
    u1_f2: S,
    u2_f3: S,
}

static G: OnceLock<Mutex<UnionU>> = OnceLock::new();

fn get_g() -> &'static Mutex<UnionU> {
    G.get().expect("Global not initialized")
}

fn f() -> S {
    let guard = get_g().lock().unwrap();
    guard.u1_f2
}

fn foo() -> i32 {
    1
}

fn main() {
    G.set(Mutex::new(UnionU {
        u1_f2: S { i: 0.0 },
        u2_f3: S { i: 0.0 },
    }))
    .unwrap();

    {
        let mut guard = get_g().lock().unwrap();
        guard.u1_f2.i = 1.0;
        guard.u2_f3.i = 1.0;
    }

    let temp1 = f();
    {
        let mut guard = get_g().lock().unwrap();
        guard.u1_f2 = temp1;
    }

    let temp2 = f();
    {
        let mut guard = get_g().lock().unwrap();
        guard.u2_f3 = temp2;
    }

    let val = foo() as f64;

    let sum = {
        let guard = get_g().lock().unwrap();
        guard.u1_f2.i + guard.u2_f3.i + val
    };

    let exit_code = if sum == 3.0 { 0 } else { 1 };
    std::process::exit(exit_code);
}