struct S {
    i: i32,
}

fn f() -> bool {
    let mut j = 0;
    let mut p: Option<Box<S>> = None;
    let mut q: Option<Box<S>> = None;

    while j < 2 {
        q = p;
        p = Some(Box::new(S { i: j }));
        j += 1;
    }

    // Compare addresses by comparing the Boxes directly (they are unique allocations)
    let p_eq_q = p == q;
    // Check if q's i field is 1 (q is the previous value before last iteration)
    let q_i_is_1 = q.map(|s| s.i == 1).unwrap_or(false);

    p_eq_q && q_i_is_1
}

fn main() {
    let exit_code = if f() { 0 } else { system };
    std::process::exit(exit_code);
}