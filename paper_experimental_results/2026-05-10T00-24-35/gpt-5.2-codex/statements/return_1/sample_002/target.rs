#[derive(Copy, Clone, Default)]
struct S {
    i: f64,
}

#[derive(Copy, Clone, Default)]
struct UnionSim {
    u1_f2: S,
    u2_f3: S,
}

fn f(g: &UnionSim) -> S {
    g.u1_f2
}

fn foo() -> i32 {
    1
}

fn main() {
    let mut g = UnionSim::default();

    g.u1_f2.i = 1.0;
    g.u2_f3.i = 1.0;

    let temp = f(&g);
    g.u1_f2 = temp;

    let temp = f(&g);
    g.u2_f3 = temp;

    let val = foo() as f64;
    let sum = g.u1_f2.i + g.u2_f3.i + val;

    if sum == 3.0 {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}