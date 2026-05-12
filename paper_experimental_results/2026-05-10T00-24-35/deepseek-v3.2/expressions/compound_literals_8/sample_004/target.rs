struct S {
    i: i32,
}

fn f() -> i32 {
    let mut p: Option<Box<S>> = None;
    let mut q: Option<Box<S>> = None;
    let mut j = 0;

    loop {
        q = p;
        p = Some(Box::new(S { i: j }));
        j += 1;
        if j >= 2 {
            break;
        }
    }

    if let (Some(p_ref), Some(q_ref)) = (&p, &q) {
        (std::ptr::eq(p_ref.as_ref(), q_ref.as_ref()) as i32) & (q_ref.i == 1) as i32
    } else {
        0
    }
}

fn main() {
    std::process::exit(f());
}