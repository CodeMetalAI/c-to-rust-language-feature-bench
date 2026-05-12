struct S {
    i: i32,
}

fn f() -> bool {
    let mut p: Option<S> = None;
    let mut q: Option<&S>;
    let mut j = 0;

    loop {
        q = p.as_ref();
        p = Some(S { i: j });
        j += 1;
        if j >= 2 {
            break;
        }
    }

    match (p.as_ref(), q) {
        (Some(p_val), Some(q_val)) => (p_val as *const _) == (q_val as *const _) && q_val.i == 1,
        _ => false,
    }
}

fn main() {
    std::process::exit(if f() { 0 } else { 1 });
}