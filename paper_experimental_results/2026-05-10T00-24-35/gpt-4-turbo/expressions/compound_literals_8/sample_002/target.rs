struct S {
    i: i32,
}

fn f() -> bool {
    let mut p: Option<Box<S>> = None;
    let mut q: Option<Box<S>> = None;
    let mut j = 0;

    loop {
        q = p.take();
        p = Some(Box::new(S { i: j }));
        j += 1;

        if j < 2 {
            continue;
        } else {
            break;
        }
    }

    match (&p, &q) {
        (Some(p_box), Some(q_box)) => std::ptr::eq(p_box.as_ref(), q_box.as_ref()) && q_box.i == 1,
        _ => false,
    }
}

fn main() {
    std::process::exit(if f() { 0 } else { 1 });
}