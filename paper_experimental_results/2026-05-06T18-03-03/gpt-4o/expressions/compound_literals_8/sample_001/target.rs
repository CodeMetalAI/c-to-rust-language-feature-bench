struct S {
    i: i32,
}

fn f() -> bool {
    let mut p: Option<S> = None;
    let mut q: Option<S>;
    let mut j = 0;

    loop {
        q = p.take();
        p = Some(S { i: j });
        j += 1;
        if j >= 2 {
            break;
        }
    }

    match (p, q) {
        (Some(ref p_val), Some(ref q_val)) => p_val.i == q_val.i + 1,
        _ => false,
    }
}

fn main() {
    std::process::exit(if f() { 0 } else { 1 });
}