struct S {
    i: i32,
}

fn f() -> bool {
    let mut p: Option<&S> = None;
    let mut q: Option<&S>;
    let mut j = 0;

    loop {
        q = p;
        let s = S { i: j };
        p = Some(&s);
        j += 1;

        if j >= 2 {
            break;
        }
    }

    match (p, q) {
        (Some(p_val), Some(q_val)) => p_val == q_val && q_val.i == 1,
        _ => false,
    }
}

fn main() {
    if !f() {
        std::process::exit(1);
    }
}