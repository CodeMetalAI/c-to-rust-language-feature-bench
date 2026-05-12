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

    if let (Some(p_ref), Some(q_ref)) = (p.as_ref(), q) {
        (p_ref as *const S == q_ref as *const S) && (q_ref.i == 1)
    } else {
        false
    }
}

fn main() {
    std::process::exit(if f() { 0 } else { 1 });
}