use std::sync::Mutex;

#[derive(Copy, Clone)]
struct S {
    i: f64,
}

struct U {
    u2_f3: S,
    u1_f2: S,
}

static G: Mutex<U> = Mutex::new(U {
    u2_f3: S { i: 0.0 },
    u1_f2: S { i: 0.0 },
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
        let mut g = G.lock().unwrap();
        g.u1_f2 = f();
    }
    {
        let mut g = G.lock().unwrap();
        g.u2_f3 = f();
    }
    let val = foo() as f64;
    let sum = {
        let g = G.lock().unwrap();
        g.u1_f2.i + g.u2_f3.i + val
    };
    if (sum == 3.0) {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}