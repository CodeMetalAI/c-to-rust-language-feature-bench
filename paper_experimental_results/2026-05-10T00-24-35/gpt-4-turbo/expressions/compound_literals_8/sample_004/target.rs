struct S {
    i: i32,
}

fn f() -> bool {
    let mut p: Option<Box<S>> = None;
    let mut q: Option<Box<S>>;
    let mut j = 0;

    loop {
        q = p;
        p = Some(Box::new(S { i: j }));
        j += 1;

        if j < 2 {
            continue;
        } else {
            break;
        }
    }

    match (&p, &q) {
        (Some(p_val), Some(q_val)) => {
            // Check if both are pointing to the same Box (which they never are due to rebinding)
            // and check if the last value of `q` is 1 (which it is when j is 1)
            Box::as_ptr(p_val) == Box::as_ptr(q_val) && q_val.i == 1
        }
        _ => false,
    }
}

fn main() {
    std::process::exit(if f() { 0 } else { 1 });
}