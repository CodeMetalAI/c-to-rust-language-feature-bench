#[derive(Debug, Default, Copy, Clone)]
struct S {
    i: f64,
}

#[derive(Debug, Default, Copy, Clone)]
union U {
    u1: (i32, S),
    u2: (S, i32),
}

fn f() -> S {
    U::u1.1
}

fn foo() -> f64 {
    1.0
}

fn main() {
    let mut g = U::u1((0, S { i: 0.0 }));
    g.u1.1.i = 1.0;
    g.u2.0.i = 1.0;
    g.u1.1 = f();
    g.u2.0 = f();
    let val = foo();
    if g.u1.1.i + g.u2.0.i + val == 3.0 {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}