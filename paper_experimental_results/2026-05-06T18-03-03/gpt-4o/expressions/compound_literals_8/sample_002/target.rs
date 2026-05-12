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

    if let (Some(p_val), Some(q_val)) = (p, q) {
        p_val.i == q_val.i && q_val.i == 1
    } else {
        false
    }
}

fn main() {
    std::process::exit(if f() { 0 } else { 1 });
}