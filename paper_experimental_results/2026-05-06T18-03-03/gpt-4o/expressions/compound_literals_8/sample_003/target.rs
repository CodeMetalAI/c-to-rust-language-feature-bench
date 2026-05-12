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
        (Some(p_ref), Some(q_ref)) => p_ref as *const _ == q_ref as *const _ && q_ref.i == 1,
        _ => false,
    }
}

fn main() {
    std::process::exit(if f() { 0 } else { 1 });
}